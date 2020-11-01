package com.github.toncherami.mpd.web.playlist.services.impl

import com.github.toncherami.mpd.web.adapter.services.MpdService
import com.github.toncherami.mpd.web.database.dto.File
import com.github.toncherami.mpd.web.playlist.services.PlaylistService
import org.springframework.stereotype.Service

@Service
class PlaylistServiceImpl(private val mpdService: MpdService) : PlaylistService {

    override fun get(): List<File> {
        return mpdService.playlistinfo().map(File::of)
    }

    override fun clear() {
        return mpdService.clear()
    }

}
