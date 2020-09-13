package com.github.toncherami.mpd.web.utils

private const val MPD_RESPONSE_KEY_VALUE_SEPARATOR = ": "

data class MpdResponse<T>(
    val version: String,
    val status: String,
    val data: T
)

private fun <T, R> MpdResponse<T>.replaceData(data: R): MpdResponse<R> {
    return MpdResponse(version, status, data)
}

fun parseResponse(response: String): MpdResponse<Map<String, String>> {
    val unwrappedResponse = parseResponseWrapper(response)

    val data = unwrappedResponse.data
        ?: return unwrappedResponse.replaceData(emptyMap())

    return data
        .trim()
        .let(::parseResponseData)
        .let(unwrappedResponse::replaceData)
}

fun parseListResponse(response: String): MpdResponse<List<Map<String, String>>> {
    val unwrappedResponse = parseResponseWrapper(response)

    val data = unwrappedResponse.data
        ?: return unwrappedResponse.replaceData(emptyList())

    val initialItemKey = getInitialListItemKey(data)

    val lookahead = "(?=$initialItemKey$MPD_RESPONSE_KEY_VALUE_SEPARATOR)".toRegex()

    return data
        .split(lookahead)
        .filter(String::isNotEmpty)
        .map(String::trim)
        .map(::parseResponseData)
        .let(unwrappedResponse::replaceData)
}

private fun getInitialListItemKey(data: String): String {
    return "^([a-zA-Z-]+)$MPD_RESPONSE_KEY_VALUE_SEPARATOR".toRegex()
        .find(data)
        ?.groupValues
        ?.get(1)
        ?: throw IllegalArgumentException("Unable to parse initial list item key")
}

private fun parseResponseWrapper(response: String): MpdResponse<String?> {
    return response
        .lines()
        // the response includes a terminating newline,
        // which produces an additional final empty string when using CharSequence.lines()
        .dropLast(1)
        .let { lines ->
            val version = lines.first()
            val status = lines.last()

            val data = lines
                .takeIf { it.count() > 2 }
                ?.drop(1)
                ?.dropLast(1)
                ?.joinToString("\n")
                ?.trim()

            MpdResponse(version, status, data)
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
