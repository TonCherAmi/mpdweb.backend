package com.github.toncherami.mpd.web.status.controllers

import com.github.toncherami.mpd.web.status.dto.Status
import com.github.toncherami.mpd.web.status.services.StatusService
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RestController

@RestController
@RequestMapping("/status")
class StatusController(private val statusService: StatusService) {

    @GetMapping
    fun status(): Status {
        return statusService.get()
    }

}
