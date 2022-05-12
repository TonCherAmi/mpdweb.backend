package com.github.toncherami.mpd.web.playback.services.impl

import com.github.toncherami.mpd.web.adapter.dto.enums.MpdSingleState
import com.github.toncherami.mpd.web.adapter.dto.enums.MpdPlaybackState
import com.github.toncherami.mpd.web.adapter.services.MpdService
import com.github.toncherami.mpd.web.adapter.utils.toMpdBooleanValue
import com.github.toncherami.mpd.web.playback.services.PlaybackService
import com.github.toncherami.mpd.web.status.dto.SingleState
import com.github.toncherami.mpd.web.status.services.StatusService
import org.springframework.stereotype.Service

@Service
class PlaybackServiceImpl(
    private val mpdService: MpdService,
    private val statusService: StatusService,
) : PlaybackService {

    override fun stop() {
        mpdService.stop()
    }

    override fun play(id: Int?) {
        mpdService.playid(id)
    }

    override fun single() {
        val state = when (statusService.get().single) {
            SingleState.ON -> MpdSingleState.OFF
            SingleState.OFF -> MpdSingleState.ONESHOT
            SingleState.ONESHOT -> MpdSingleState.ON
        }

        mpdService.single(state.toValue())
    }

    override fun repeat() {
        val state = !statusService.get().repeat

        mpdService.repeat(state.toMpdBooleanValue())
    }

    override fun consume() {
        val state = !statusService.get().consume

        mpdService.consume(state.toMpdBooleanValue())
    }

    override fun random() {
        val state = !statusService.get().random

        mpdService.random(state.toMpdBooleanValue())
    }

    override fun toggle() {
        val status = mpdService.status()

        if (status.state == MpdPlaybackState.PLAYING) {
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
