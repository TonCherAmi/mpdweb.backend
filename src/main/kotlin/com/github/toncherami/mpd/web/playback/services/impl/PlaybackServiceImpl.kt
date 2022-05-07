package com.github.toncherami.mpd.web.playback.services.impl

import com.github.toncherami.mpd.web.adapter.dto.enums.MpdState
import com.github.toncherami.mpd.web.adapter.services.MpdService
import com.github.toncherami.mpd.web.playback.services.PlaybackService
import org.springframework.stereotype.Service

@Service
class PlaybackServiceImpl(private val mpdService: MpdService) : PlaybackService {

    override fun stop() {
        mpdService.stop()
    }

    override fun play(id: Int?) {
        mpdService.playid(id)
    }

    override fun toggle() {
        val status = mpdService.status()

        if (status.state == MpdState.PLAYING) {
            return mpdService.pause()
        }

        mpdService.play()
    }

    override fun next() {
        mpdService.next()
    }

    override fun prev() {
        mpdService.previous()
    }

    override fun seek(time: Double) {
        mpdService.seekcur(time.toString())
    }

    override fun seekBack(time: Double) {
        mpdService.seekcur("-$time")
    }

    override fun seekForward(time: Double) {
        mpdService.seekcur("+$time")
    }

}
