package com.github.toncherami.mpd.web.volume.services.impl

import com.github.toncherami.mpd.web.adapter.services.MpdService
import com.github.toncherami.mpd.web.volume.services.VolumeService
import org.springframework.stereotype.Service

@Service
class VolumeServiceImpl(
    private val mpdService: MpdService,
) : VolumeService {

    override fun set(volume: Int) {
        mpdService.setvol(volume)
    }

}
