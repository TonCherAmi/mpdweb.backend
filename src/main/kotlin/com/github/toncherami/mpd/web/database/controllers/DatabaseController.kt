package com.github.toncherami.mpd.web.database.controllers

import com.github.toncherami.mpd.web.database.data.DatabaseCount
import com.github.toncherami.mpd.web.database.data.DatabaseItem
import com.github.toncherami.mpd.web.database.data.enums.DatabaseCoverType
import com.github.toncherami.mpd.web.database.services.DatabaseService
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.PostMapping
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RequestParam
import org.springframework.web.bind.annotation.RestController

@RestController
@RequestMapping("/database")
class DatabaseController(private val databaseService: DatabaseService) {

    @GetMapping
    fun database(@RequestParam uri: String): List<DatabaseItem> {
        return databaseService.get(uri)
    }

    @GetMapping("/count")
    fun count(@RequestParam uri: String): DatabaseCount {
        return databaseService.count(uri)
    }

    @GetMapping("/search")
    fun search(@RequestParam term: String): List<DatabaseItem> {
        return databaseService.search(term)
    }

    @GetMapping("/cover/directory", produces = ["image/*"])
    fun coverDirectory(@RequestParam uri: String): ByteArray {
        return databaseService.cover(uri, DatabaseCoverType.DIRECTORY)
    }

    @GetMapping("/cover/embedded", produces = ["image/*"])
    fun coverEmbedded(@RequestParam uri: String): ByteArray {
        return databaseService.cover(uri, DatabaseCoverType.EMBEDDED)
    }

    @PostMapping("/update")
    fun update() {
        databaseService.update()
    }

}
