package com.github.toncherami.mpd.web.playlist.services.impl

import com.github.toncherami.mpd.web.adapter.services.MpdService
import com.github.toncherami.mpd.web.database.dto.DatabaseFile
import com.github.toncherami.mpd.web.playlist.dto.PlaylistItem
import com.github.toncherami.mpd.web.playlist.services.PlaylistService
import org.springframework.stereotype.Service

@Service
class PlaylistServiceImpl(private val mpdService: MpdService) : PlaylistService {

    override fun get(): List<PlaylistItem> {
        return mpdService.playlistinfo().map(PlaylistItem::of)
    }

    override fun add(uri: String) {
        mpdService.add(uri)
    }

    override fun clear() {
        mpdService.clear()
    }

    override fun replace(uri: String) {
        mpdService.commandList {
            clear()
            add(uri)
            play()
        }
    }

    override fun delete(id: Int) {
        mpdService.deleteid(id)
    }

}
