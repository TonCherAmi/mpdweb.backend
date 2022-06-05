package com.github.toncherami.mpd.web.adapter.deserializers

import com.github.toncherami.mpd.web.adapter.deserializers.data.MpdError
import com.github.toncherami.mpd.web.adapter.deserializers.data.MpdResponse
import com.github.toncherami.mpd.web.adapter.deserializers.data.enums.MpdErrorCode
import com.github.toncherami.mpd.web.adapter.deserializers.exceptions.MpdDeserializationException
import com.github.toncherami.mpd.web.common.data.Either
import org.springframework.core.serializer.Deserializer
import org.springframework.integration.ip.tcp.serializer.SoftEndOfStreamException
import java.io.ByteArrayOutputStream
import java.io.DataInputStream
import java.io.IOException
import java.io.InputStream

const val MPD_RESPONSE_OK_TERMINATOR = "OK"

private const val MPD_RESPONSE_BINARY_KEY = "binary"

private const val UTF8_NEWLINE = 0X0A

class MpdDeserializer : Deserializer<Either<MpdError, MpdResponse>> {

    override fun deserialize(inputStream: InputStream): Either<MpdError, MpdResponse> {
        val dataInputStream = DataInputStream(inputStream)

        val pairs = mutableListOf<Pair<String, String>>()

        val binary = mutableListOf<ByteArray>()

        while (true) {
            try {
                val line = dataInputStream.readMpdLine()

                if (line == MPD_RESPONSE_OK_TERMINATOR) {
                    break
                }

                if (line.matches(mpdVersionRegex)) {
                    continue
                }

                if (line.matches(mpdErrorRegex)) {
                    return Either.Left(
                        parseMpdError(line)
                    )
                }

                val pair = parseMpdLine(line)

                pairs.add(pair)

                if (pair.first == MPD_RESPONSE_BINARY_KEY) {
                    val n = pair.second.toInt()

                    binary.add(
                        dataInputStream.readNBytes(n)
                    )

                    // skip the newline
                    dataInputStream.skip(1)
                }
            } catch (exception: IOException) {
                throw SoftEndOfStreamException()
            }
        }

        return Either.Right(
            MpdResponse(
                pairs = pairs,
                binary = binary,
            )
        )
    }

    private fun DataInputStream.readMpdLine(): String {
        val outputStream = ByteArrayOutputStream()

        var byte = read()

        while (byte != UTF8_NEWLINE) {
            if (byte == -1) {
                throw SoftEndOfStreamException()
            }

            outputStream.write(byte)

            byte = read()
        }

        return outputStream.toString("UTF-8")
    }

    private fun parseMpdError(line: String): MpdError {
        val groups = mpdErrorRegex.find(line)
            ?.groups
            ?: throw MpdDeserializationException(what = "MPD error", from = line)

        val code = groups["code"]?.value?.toInt()
            ?: throw MpdDeserializationException(what = "MPD error code", from = line)

        val message = groups["message"]?.value.orEmpty()

        val command = groups["command"]?.value?.takeUnless { it.isBlank() }

        val commandIndex = groups["commandIndex"]?.value?.toInt()
            ?: throw MpdDeserializationException(what = "MPD command index", from = line)

        return MpdError(
            code = MpdErrorCode.values().find { it.value == code }
                ?: MpdErrorCode.UNK,
            message = message,
            command = command,
            commandIndex = commandIndex
        )
    }

    private fun parseMpdLine(line: String): Pair<String, String> {
        val groups = mpdLineRegex.find(line)
            ?.groups
            ?: throw MpdDeserializationException(what = "MPD line", from = line)

        val key = groups["key"]?.value
            ?: throw MpdDeserializationException(what = "MPD line key", from = line)

        val value = groups["value"]?.value
            ?: throw MpdDeserializationException(what = "MPD line value", from = line)

        return key to value
    }

    private companion object {

        val mpdLineRegex = Regex("^(?<key>[^: ]+): (?<value>.+)\$")

        val mpdErrorRegex = Regex(
            "ACK \\[(?<code>\\d+)@(?<commandIndex>\\d+)] \\{(?<command>\\w*)} (?<message>.*)"
        )

        val mpdVersionRegex = Regex("OK MPD [\\d.]+")

    }

}
