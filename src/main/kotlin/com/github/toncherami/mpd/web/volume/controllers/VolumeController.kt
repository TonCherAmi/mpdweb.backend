package com.github.toncherami.mpd.web.volume.controllers

import com.github.toncherami.mpd.web.volume.data.api.request.VolumeSetBody
import com.github.toncherami.mpd.web.volume.data.api.request.enums.VolumeSetMode
import com.github.toncherami.mpd.web.volume.services.VolumeService
import org.springframework.web.bind.annotation.PostMapping
import org.springframework.web.bind.annotation.RequestBody
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RestController
import javax.validation.Valid

@RestController
@RequestMapping("/volume")
class VolumeController(private val volumeService: VolumeService) {

    @PostMapping
    fun set(@Valid @RequestBody body: VolumeSetBody) {
        when (body.mode) {
            VolumeSetMode.INC -> volumeService.inc(body.volume)
            VolumeSetMode.DEC -> volumeService.dec(body.volume)
            VolumeSetMode.ABSOLUTE -> volumeService.set(body.volume)
        }
    }

}
