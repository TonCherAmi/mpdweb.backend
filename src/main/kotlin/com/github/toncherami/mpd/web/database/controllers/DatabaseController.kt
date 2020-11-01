package com.github.toncherami.mpd.web.database.controllers

import com.github.toncherami.mpd.web.database.dto.DatabaseItem
import com.github.toncherami.mpd.web.database.services.DatabaseService
import com.github.toncherami.mpd.web.database.dto.api.request.DatabaseGetBody
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.PostMapping
import org.springframework.web.bind.annotation.RequestBody
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RequestParam
import org.springframework.web.bind.annotation.RestController

@RestController
@RequestMapping("/database")
class DatabaseController(private val databaseService: DatabaseService) {

    @GetMapping
    fun database(
        @RequestParam
        uri: String
    ): List<DatabaseItem> {
        return databaseService.get(uri)
    }

    @PostMapping("/update")
    fun update() {
        databaseService.update()
    }

}
