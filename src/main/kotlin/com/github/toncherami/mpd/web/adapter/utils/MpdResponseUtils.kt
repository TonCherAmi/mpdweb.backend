package com.github.toncherami.mpd.web.adapter.utils

const val MPD_OK_RESPONSE_STATUS = "OK"
const val MPD_ERROR_RESPONSE_STATUS = "ACK"

private const val MPD_RESPONSE_KEY_VALUE_SEPARATOR = ": "

data class MpdResponse<T>(val status: String, val data: T)

private fun <T, R> MpdResponse<T>.replaceData(data: R): MpdResponse<R> {
    return MpdResponse(status, data)
}

fun parseResponse(response: String): MpdResponse<Map<String, String>> {
    val unwrappedResponse = parseResponseWrapper(response)

    val data = unwrappedResponse.data
        ?: return unwrappedResponse.replaceData(emptyMap())

    return data
        .let(::parseResponseData)
        .let(unwrappedResponse::replaceData)
}

fun parseListResponse(response: String, keys: List<String>): MpdResponse<List<Map<String, String>>> {
    val unwrappedResponse = parseResponseWrapper(response)

    val data = unwrappedResponse.data
        ?: return unwrappedResponse.replaceData(emptyList())

    val lookahead = getLookahead(keys)

    return data
        .split(lookahead)
        .filter(String::isNotEmpty)
        .map(String::trim)
        .map(::parseResponseData)
        .let(unwrappedResponse::replaceData)
}

private fun getLookahead(keys: List<String>): Regex {
    if (keys.isEmpty()) {
        throw IllegalArgumentException("Key list cannot be empty")
    }

    val condition = keys
        .takeUnless { it.count() == 1 }
        ?.joinToString("$MPD_RESPONSE_KEY_VALUE_SEPARATOR|")
        ?: keys.first().plus(MPD_RESPONSE_KEY_VALUE_SEPARATOR)

    return "(?=$condition)".toRegex()
}

private fun parseResponseWrapper(response: String): MpdResponse<String?> {
    return response.lines()
        .let { lines ->
            val status = lines.last()

            val data = lines
                .takeIf { it.count() > 1 }
                ?.dropLast(1)
                ?.joinToString("\n")

            MpdResponse(status, data)
        }
}

private fun parseResponseData(response: String): Map<String, String> {
    return response
        .lines()
        .map { it.split(MPD_RESPONSE_KEY_VALUE_SEPARATOR.toRegex(), 2) }
        .also { splitLines ->
            if (splitLines.any { it.count() != 2 }) {
                throw IllegalArgumentException("Malformed MPD key-value response line")
            }
        }
        .map { (key, value) ->
            key to value
        }
        .toMap()
}
