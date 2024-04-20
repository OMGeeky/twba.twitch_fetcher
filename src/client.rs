use crate::prelude::*;
use twba_backup_config::Conf;
use twba_local_db::entities::users::Model;
use twba_local_db::entities::videos::ActiveModel;
use twba_local_db::prelude::{Status, Users, UsersColumn, Videos, VideosColumn, VideosModel};
use twba_local_db::re_exports::sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use twba_twitch_data::TwitchClient;

pub struct FetcherClient<'a> {
    pub conf: Conf,
    pub db: DatabaseConnection,
    pub twitch_client: TwitchClient<'a>,
}

impl<'a> FetcherClient<'a> {
    pub(crate) fn new(conf: Conf, db: DatabaseConnection, twitch_client: TwitchClient<'a>) -> Self {
        Self {
            conf,
            db,
            twitch_client,
        }
    }
}

impl<'a> FetcherClient<'a> {
    pub(crate) async fn fetch_new_videos(&self) -> Result<()> {
        let users = Users::find()
            .filter(UsersColumn::Active.eq(true))
            .all(&self.db)
            .await?;
        info!("Fetching videos for {} users", users.len());
        for user in users {
            match self.fetch_videos_for_user(&user).await {
                Ok(_) => {
                    info!("Fetched videos for user: {}", user.twitch_name);
                }
                Err(e) => {
                    error!(
                        "Could not fetch videos for user: {} because of error: {:?}",
                        user.twitch_name, e
                    );
                }
            }
        }
        info!("Done fetching videos for all users");
        Ok(())
    }

    async fn fetch_videos_for_user(&self, user: &Model) -> Result<()> {
        info!("Fetching videos for user: '{}'", user.twitch_name);
        let videos = self
            .twitch_client
            .get_videos_from_login(&user.twitch_id, None)
            .await
            .map_err(FetcherError::GetVideosError)?;
        for video in videos {
            info!("Adding video: {} to the database", video.title);
            let existing_video_found = Videos::find()
                .filter(VideosColumn::TwitchId.eq(video.id.to_string()))
                .one(&self.db)
                .await
                .is_ok();
            if existing_video_found {
                info!("Video with id: {} already exists in the database", video.id);
                continue;
            }

            ActiveModel {
                id: ActiveValue::NotSet,
                twitch_id: ActiveValue::Set(video.id.to_string()),
                name: ActiveValue::Set(video.title),
                user_id: ActiveValue::Set(user.id),
                created_at: ActiveValue::Set(video.created_at.to_rfc3339()),
                youtube_id: ActiveValue::NotSet,
                youtube_playlist_name: ActiveValue::NotSet,
                youtube_preview_image_url: ActiveValue::NotSet,
                youtube_playlist_id: ActiveValue::NotSet,
                duration: ActiveValue::Set(video.duration as i32),
                twitch_download_url: ActiveValue::Set(Some(video.url)),
                status: ActiveValue::Set(Status::NotStarted),
                fail_count: ActiveValue::NotSet,
                part_count: ActiveValue::Set(0),
                twitch_preview_image_url: ActiveValue::NotSet,
                youtube_playlist_created_at: ActiveValue::NotSet,
                fail_reason: ActiveValue::NotSet,
            }
            .insert(&self.db)
            .await?;
        }
        Ok(())
    }
}
