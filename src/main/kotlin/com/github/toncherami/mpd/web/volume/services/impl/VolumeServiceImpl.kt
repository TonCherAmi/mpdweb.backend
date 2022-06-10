package com.github.toncherami.mpd.web.volume.services.impl

import com.github.toncherami.mpd.web.adapter.services.MpdService
import com.github.toncherami.mpd.web.status.services.StatusService
import com.github.toncherami.mpd.web.volume.services.VolumeService
import org.springframework.stereotype.Service
import kotlin.math.max
import kotlin.math.min

@Service
class VolumeServiceImpl(
    private val mpdService: MpdService,
    private val statusService: StatusService,
) : VolumeService {

    override fun set(volume: Int) {
        mpdService.setvol(volume)
    }

    override fun inc(volume: Int) {
        val value = min(100, statusService.get().volume.plus(volume))

        mpdService.setvol(value)
    }

    override fun dec(volume: Int) {
        val value = max(0, statusService.get().volume.minus(volume))

        mpdService.setvol(value)
    }

}
