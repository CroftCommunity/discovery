package com.croftc.p2pexp

import android.os.Bundle
import android.widget.Button
import android.widget.EditText
import android.widget.LinearLayout
import android.widget.ScrollView
import android.widget.TextView
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import androidx.lifecycle.lifecycleScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import org.json.JSONObject
import uniffi.group_core.GroupClient

/**
 * Truly basic read/post UI over the Rust iroh + Automerge core.
 *
 * The entire bridge to Rust is [GroupClient.handle], a single JSON-in / JSON-out
 * call. Because that call blocks while iroh work runs on the core's embedded Tokio
 * runtime (Delta Chat's "prefer blocking calls dispatched on a background thread"
 * guidance), every invocation here is dispatched on [Dispatchers.IO].
 */
class MainActivity : AppCompatActivity() {

    // One client per session. Holds the iroh node + Automerge doc inside Rust.
    private val client: GroupClient by lazy { GroupClient() }

    private lateinit var messagesView: TextView
    private lateinit var messageInput: EditText
    private lateinit var inviteInput: EditText

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(buildUi())

        // Set a display name, then this device creates a fresh group. Build the
        // payload via JSONObject so Build.MODEL is escaped (it can contain quotes).
        lifecycleScope.launch {
            send(JSONObject().put("cmd", "init").put("author", "android-${android.os.Build.MODEL}").toString())
            send("""{"cmd":"create_group"}""")
            refresh()
        }
    }

    override fun onDestroy() {
        // GroupClient owns native state (the iroh node + Automerge doc). Release it
        // so a recreated Activity doesn't leak the previous session's resources.
        client.close()
        super.onDestroy()
    }

    private fun buildUi(): LinearLayout {
        val root = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            setPadding(32, 32, 32, 32)
        }

        messagesView = TextView(this).apply { text = "(no messages yet)" }
        root.addView(ScrollView(this).apply { addView(messagesView) },
            LinearLayout.LayoutParams(LinearLayout.LayoutParams.MATCH_PARENT, 0, 1f))

        messageInput = EditText(this).apply { hint = "Type a message" }
        root.addView(messageInput)

        root.addView(Button(this).apply {
            text = "Post"
            setOnClickListener {
                val text = messageInput.text.toString().ifBlank { return@setOnClickListener }
                messageInput.text.clear()
                lifecycleScope.launch {
                    send(JSONObject().put("cmd", "post_message").put("text", text).toString())
                    refresh()
                }
            }
        })

        root.addView(Button(this).apply {
            text = "Show invite"
            setOnClickListener {
                lifecycleScope.launch {
                    val resp = JSONObject(send("""{"cmd":"get_invite"}"""))
                    val invite = resp.optString("invite", resp.optString("message"))
                    Toast.makeText(this@MainActivity, invite, Toast.LENGTH_LONG).show()
                }
            }
        })

        inviteInput = EditText(this).apply { hint = "Paste an invite to join" }
        root.addView(inviteInput)

        root.addView(Button(this).apply {
            text = "Join group"
            setOnClickListener {
                val invite = inviteInput.text.toString().ifBlank { return@setOnClickListener }
                lifecycleScope.launch {
                    send(JSONObject().put("cmd", "join_group").put("invite", invite).toString())
                    refresh()
                }
            }
        })

        return root
    }

    /** Refresh the message list from the core. */
    private suspend fun refresh() {
        val resp = JSONObject(send("""{"cmd":"get_messages"}"""))
        val msgs = resp.optJSONArray("messages") ?: return
        val rendered = buildString {
            for (i in 0 until msgs.length()) {
                val m = msgs.getJSONObject(i)
                append(m.getString("author")).append(": ").append(m.getString("text")).append('\n')
            }
        }
        messagesView.text = rendered.ifBlank { "(no messages yet)" }
    }

    /** Dispatch a JSON command to the Rust core off the main thread. */
    private suspend fun send(commandJson: String): String =
        withContext(Dispatchers.IO) { client.handle(commandJson) }
}
