package com.github.toncherami.mpd.web.database.services.controllers

import com.github.toncherami.mpd.web.database.services.DatabaseService
import org.springframework.web.bind.annotation.PostMapping
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RestController

@RestController
@RequestMapping("/database")
class DatabaseController(private val databaseService: DatabaseService) {

    @PostMapping("/update")
    fun update() {
        databaseService.update()
    }

}
