package com.github.toncherami.mpd.web.common.handlers

import com.github.toncherami.mpd.web.adapter.deserializers.data.enums.MpdErrorCode
import com.github.toncherami.mpd.web.adapter.exceptions.MpdException
import org.springframework.http.HttpHeaders
import org.springframework.http.HttpStatus
import org.springframework.http.ResponseEntity
import org.springframework.web.bind.annotation.ControllerAdvice
import org.springframework.web.bind.annotation.ExceptionHandler
import org.springframework.web.servlet.mvc.method.annotation.ResponseEntityExceptionHandler

@ControllerAdvice
class MpdExceptionHandler : ResponseEntityExceptionHandler() {

    data class Error(
        val code: Int,
        val message: String?,
    )

    @ExceptionHandler(MpdException::class)
    fun handleMpdException(mpdException: MpdException): ResponseEntity<Any> {
        val status = if (mpdException.code == MpdErrorCode.NO_EXIST) {
            HttpStatus.NOT_FOUND
        } else {
            HttpStatus.INTERNAL_SERVER_ERROR
        }

        return ResponseEntity(
            Error(code = mpdException.code.value,message = mpdException.message),
            HttpHeaders(),
            status,
        )
    }

}
