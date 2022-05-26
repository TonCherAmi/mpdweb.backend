package com.github.toncherami.mpd.web.queue.controllers

import com.github.toncherami.mpd.web.queue.data.QueueItem
import com.github.toncherami.mpd.web.queue.data.api.request.QueueAddBody
import com.github.toncherami.mpd.web.queue.data.api.request.QueueDeleteBody
import com.github.toncherami.mpd.web.queue.data.api.request.QueueReplaceBody
import com.github.toncherami.mpd.web.queue.services.QueueService
import org.springframework.web.bind.annotation.DeleteMapping
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.PostMapping
import org.springframework.web.bind.annotation.PutMapping
import org.springframework.web.bind.annotation.RequestBody
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RestController

@RestController
@RequestMapping("/queue")
class QueueController(private val queueService: QueueService) {

    @GetMapping
    fun get(): List<QueueItem> {
        return queueService.get()
    }

    @PostMapping
    fun add(@RequestBody body: QueueAddBody) {
        queueService.add(body.source)
    }

    @DeleteMapping
    fun delete(@RequestBody body: QueueDeleteBody) {
        if (body.id == null) {
            queueService.clear()

            return
        }

        queueService.delete(id = body.id)
    }

    @PutMapping
    fun replace(@RequestBody body: QueueReplaceBody) {
        queueService.replace(body.source)
    }

}
