use objc2::msg_send;
use objc2::rc::autoreleasepool;
use objc2::runtime::{AnyClass, AnyObject};
use objc2_foundation::{NSNumber, NSString};
use objc2_media_player::{
    MPMediaItemPropertyAlbumTitle,
    MPMediaItemPropertyArtist,
    MPMediaItemPropertyBookmarkTime, // use BookmarkTime if PlaybackTime isn't available
    MPMediaItemPropertyPlaybackDuration,
    MPMediaItemPropertyTitle,
    MPNowPlayingInfoCenter,
    MPNowPlayingPlaybackState,
};
use std::ffi::CString;
use std::ptr;

// Set now playing: title, artist, album, duration. current_time is optional.
pub fn set_now_playing(
    title: &str,
    artist: &str,
    album: &str,
    duration: f64,
    _current_time: Option<f64>,
) {
    autoreleasepool(|_pool| {
        let title_str = NSString::from_str(title);
        let artist_str = NSString::from_str(artist);
        let album_str = NSString::from_str(album);
        let duration_num = NSNumber::new_f64(duration);
        let time = NSNumber::new_f64(0.0);

        unsafe {
            // Look up NSMutableDictionary
            let dict_class =
                AnyClass::get(&CString::new("NSMutableDictionary").unwrap().as_c_str()).unwrap();
            let dict: *mut AnyObject = msg_send![dict_class, alloc];
            let dict: *mut AnyObject = msg_send![dict, init];

            let _: () = msg_send![dict, setObject:&*title_str, forKey:MPMediaItemPropertyTitle];
            let _: () = msg_send![dict, setObject:&*artist_str, forKey:MPMediaItemPropertyArtist];
            let _: () =
                msg_send![dict, setObject:&*album_str, forKey:MPMediaItemPropertyAlbumTitle];
            let _: () = msg_send![dict, setObject:&*duration_num, forKey:MPMediaItemPropertyPlaybackDuration];

            let _: () = msg_send![dict, setObject:&*time, forKey:MPMediaItemPropertyBookmarkTime];

            // Optional: if you want to set playback time, ensure you’re using BookmarkTime
            // if let Some(t) = _current_time {
            //     let t_num = NSNumber::new_f64(t);
            //     msg_send![dict, setObject:&*t_num forKey:MPMediaItemPropertyBookmarkTime];
            // }

            let info_center = MPNowPlayingInfoCenter::defaultCenter();
            println!("info_center: {:?}", info_center);
            let info_center_ref = &*info_center;

            // clear it out first?
            let nil: *const AnyObject = ptr::null();
            let _: () = msg_send![info_center_ref, setNowPlayingInfo: nil];

            let _: () = msg_send![info_center_ref, setNowPlayingInfo:dict];
        }
    });
}

pub fn update_playback_state(is_playing: bool) {
    autoreleasepool(|_pool| {
        let info_center = unsafe { MPNowPlayingInfoCenter::defaultCenter() };
        unsafe {
            info_center.setPlaybackState(if is_playing {
                MPNowPlayingPlaybackState::Playing
            } else {
                MPNowPlayingPlaybackState::Paused
            });
        }
    });
}

pub fn update_playback_time(current_time: f64) {
    autoreleasepool(|_pool| {
        let info_center = unsafe { MPNowPlayingInfoCenter::defaultCenter() };
        unsafe {
            // Create a new dictionary and set BookmarkTime
            let dict_class =
                AnyClass::get(&CString::new("NSMutableDictionary").unwrap().as_c_str()).unwrap();
            let dict: *mut AnyObject = msg_send![dict_class, alloc];
            let dict: *mut AnyObject = msg_send![dict, init];

            let time = NSNumber::new_f64(current_time);
            let _: () = msg_send![dict, setObject:&*time, forKey:MPMediaItemPropertyBookmarkTime];

            info_center.setNowPlayingInfo(Some(&*(dict as *const _)));
        }
    });
}
