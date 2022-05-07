package com.github.toncherami.mpd.web.adapter.interceptors

import com.github.toncherami.mpd.web.adapter.properties.MpdProperties
import com.github.toncherami.mpd.web.adapter.utils.MpdCommand
import com.github.toncherami.mpd.web.adapter.utils.MpdCommandBuilder
import com.github.toncherami.mpd.web.common.data.Either
import org.springframework.integration.ip.tcp.connection.TcpConnectionInterceptorSupport
import org.springframework.integration.support.MessageBuilder
import org.springframework.messaging.Message
import org.springframework.messaging.MessagingException
import java.util.concurrent.Semaphore
import java.util.concurrent.TimeUnit

private const val MPD_HANDSHAKE_TIMEOUT = 10L // seconds

class MpdHandshakeInterceptor(
    private val mpdProperties: MpdProperties
) : TcpConnectionInterceptorSupport() {

    private val handshakeSemaphore = Semaphore(0)

    @Volatile
    private var hasReceivedClose = false

    @Volatile
    private var isSendPending = false

    @Volatile
    private var isHandshakeComplete = false

    private val isAuthenticationRequired = mpdProperties.password?.isNotEmpty()
        ?: false

    override fun onMessage(message: Message<*>): Boolean {
        if (!isHandshakeComplete) {
            synchronized(this) {
                if (message.isValidHandshakeResponse()) {
                    handleHandshakeSuccess()
                } else {
                    handleHandshakeFailure()
                }

                return true
            }
        }

        try {
            return super.onMessage(message)
        } finally {
            if (!isSendPending) {
                checkDeferredClose()
            }
        }
    }

    private fun checkDeferredClose() {
        if (this.hasReceivedClose) {
            close()
        }
    }

    private fun handleHandshakeFailure() {
        throw MessagingException("Handshake failure")
    }

    private fun handleHandshakeSuccess() {
        isHandshakeComplete = true

        handshakeSemaphore.release()
    }

    override fun send(message: Message<*>) {
        isSendPending = true

        try {
            if (!isHandshakeComplete) {
                sendHandshakeMessage()

                try {
                    handshakeSemaphore.tryAcquire(MPD_HANDSHAKE_TIMEOUT, TimeUnit.SECONDS)
                } catch (_: InterruptedException) {
                    Thread.currentThread().interrupt()
                }

                if (!isHandshakeComplete) {
                    throw MessagingException("Unexpected handshake error")
                }
            }

            super.send(message)
        } finally {
            isSendPending = false

            checkDeferredClose()
        }
    }

    private fun sendHandshakeMessage() {
        val message = getHandshakeMessage()

        super.send(message)
    }

    private fun getHandshakeMessage(): Message<String> {
        if (isAuthenticationRequired) {
            return getAuthenticationMessage()
        }

        return getPingMessage()
    }

    private fun getPingMessage(): Message<String> {
        val command = MpdCommandBuilder.command(MpdCommand.PING)
            .build()

        return MessageBuilder.withPayload(command).build()
    }

    private fun getAuthenticationMessage(): Message<String> {
        val command = MpdCommandBuilder.command(MpdCommand.PASSWORD)
            .argument(mpdProperties.password!!)
            .build()

        return MessageBuilder.withPayload(command).build()
    }

    override fun close() {
        if (isHandshakeComplete && !isSendPending) {
            return super.close()
        }

        hasReceivedClose = true
    }

    private fun Message<*>.isValidHandshakeResponse(): Boolean {
        return payload is Either.Right<*>
    }

}
