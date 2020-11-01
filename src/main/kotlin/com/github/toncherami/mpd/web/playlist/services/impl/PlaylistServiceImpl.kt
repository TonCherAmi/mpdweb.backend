package com.github.toncherami.mpd.web.playlist.services.impl

import com.github.toncherami.mpd.web.adapter.dto.MpdPlaylistItem
import com.github.toncherami.mpd.web.adapter.services.MpdService
import com.github.toncherami.mpd.web.common.utils.toDuration
import com.github.toncherami.mpd.web.playlist.dto.PlaylistItem
import com.github.toncherami.mpd.web.playlist.services.PlaylistService
import org.springframework.stereotype.Service
import java.util.concurrent.TimeUnit

@Service
class PlaylistServiceImpl(private val mpdService: MpdService) : PlaylistService {

    override fun get(): List<PlaylistItem> {
        return mpdService.playlistinfo().map(MpdPlaylistItem::toDto)
    }

    override fun clear() {
        return mpdService.clear()
    }

}

private fun MpdPlaylistItem.toDto() = PlaylistItem(
    id = id,
    position = position,
    file = file,
    artist = artist,
    title = title,
    duration = duration.toDuration(TimeUnit.SECONDS)
)
