package com.github.toncherami.mpd.web.adapter.deserializers

import com.github.toncherami.mpd.web.adapter.deserializers.data.MpdError
import com.github.toncherami.mpd.web.adapter.deserializers.exceptions.MpdDeserializationException
import com.github.toncherami.mpd.web.common.data.Either
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows
import java.io.ByteArrayOutputStream

internal class MpdDeserializerTest {

    private val deserializer = MpdDeserializer()

    @Test
    fun deserializeShouldSucceedForSingleValue() {
        val inputStream = """
            OK MPD 0.23.5
            test: value
            OK

        """.trimIndent().encodeToByteArray().inputStream()

        val result = deserializer.deserialize(inputStream)

        assertTrue(result is Either.Right)

        result as Either.Right

        assertTrue(result.value.binary.isEmpty())

        assertEquals(
            listOf(
                "test" to "value",
            ),
            result.value.pairs
        )
    }

    @Test
    fun deserializeShouldSucceedForMultipleValues() {
        val inputStream = """
            OK MPD 0.23.5
            test: value
            test1: value1
            test2: value2
            test3: value3
            OK

        """.trimIndent().encodeToByteArray().inputStream()

        val result = deserializer.deserialize(inputStream)

        assertTrue(result is Either.Right)

        result as Either.Right

        assertTrue(result.value.binary.isEmpty())

        assertEquals(
            listOf(
                "test" to "value",
                "test1" to "value1",
                "test2" to "value2",
                "test3" to "value3",
            ),
            result.value.pairs
        )
    }

    @Test
    fun deserializeShouldSucceedForBinary() {
        val binary = byteArrayOf(0x1, 0x2, 0x3, 0x4, 0x5)

        val outputStream = ByteArrayOutputStream()

        val partA = """
            OK MPD 0.23.5
            size: 5
            binary: 5

        """.trimIndent().encodeToByteArray()

        val partB = """

            OK

        """.trimIndent().encodeToByteArray()

        outputStream.writeBytes(partA)
        outputStream.writeBytes(binary)
        outputStream.writeBytes(partB)

        val result = deserializer.deserialize(outputStream.toByteArray().inputStream())

        assertTrue(result is Either.Right)

        result as Either.Right

        assertEquals(1, result.value.binary.size)

        assertTrue(
            binary.contentEquals(result.value.binary.first()),
        )

        assertEquals(
            listOf(
                "size" to "5",
                "binary" to "5",
            ),
            result.value.pairs,
        )
    }

    @Test
    fun deserializeShouldFailForMalformedLine() {
        val inputStream = """
            OK MPD 0.23.5
            test value
            OK

        """.trimIndent().encodeToByteArray().inputStream()

        assertThrows<MpdDeserializationException> {
            deserializer.deserialize(inputStream)
        }
    }

    @Test
    fun deserializeShouldSucceedForError() {
        val inputStream = """
            OK MPD 0.23.5
            ACK [5@0] {} unknown command "test"

        """.trimIndent().encodeToByteArray().inputStream()

        val result = deserializer.deserialize(inputStream)

        assertTrue(result is Either.Left)

        result as Either.Left

        assertEquals(
            MpdError(
                code = 5,
                message = "unknown command \"test\"",
                command = null,
                commandIndex = 0,
            ),
            result.value,
        )
    }

    @Test
    fun deserializeShouldFailForMalformedError() {
        val inputStream = """
            OK MPD 0.23.5
            !CK [5@0] {} unknown command "test"

        """.trimIndent().encodeToByteArray().inputStream()

        assertThrows<MpdDeserializationException> {
            deserializer.deserialize(inputStream)
        }
    }

}
