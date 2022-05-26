package com.github.toncherami.mpd.web.playlists.services.impl

import com.github.toncherami.mpd.web.adapter.services.MpdService
import com.github.toncherami.mpd.web.database.data.DatabaseFile
import com.github.toncherami.mpd.web.playlists.data.Playlist
import com.github.toncherami.mpd.web.playlists.services.PlaylistService
import org.springframework.stereotype.Service

@Service
class PlaylistServiceImpl(
    private val mpdService: MpdService,
) : PlaylistService {

    override fun get(): List<Playlist> {
        return mpdService.listplaylists().map(Playlist::of)
    }

    override fun delete(name: String) {
        mpdService.rm(name)
    }

    override fun rename(from: String, to: String) {
        mpdService.rename(name = from, newName = to)
    }

    override fun getFiles(name: String): List<DatabaseFile> {
        return mpdService.listplaylistinfo(name).map(DatabaseFile::of)
    }

    override fun addTracks(name: String, uri: String) {
        mpdService.playlistadd(name, uri)
    }

    override fun deleteFiles(name: String, positions: List<Int>) {
        mpdService.commandList {
            positions.forEach {
                playlistdelete(name, it)
            }
        }
    }

}
