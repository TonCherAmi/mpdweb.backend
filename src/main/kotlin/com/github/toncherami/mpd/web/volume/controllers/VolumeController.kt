package com.github.toncherami.mpd.web.volume.controllers

import com.github.toncherami.mpd.web.volume.data.api.request.VolumeSetBody
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
        volumeService.set(body.volume)
    }

}
