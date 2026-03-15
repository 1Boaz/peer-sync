import com.intellij.ide.util.PropertiesComponent
import com.intellij.openapi.actionSystem.ActionManager
import com.intellij.openapi.actionSystem.AnAction
import com.intellij.openapi.actionSystem.AnActionEvent
import com.intellij.openapi.fileChooser.FileChooser
import com.intellij.openapi.fileChooser.FileChooserDescriptor
import com.intellij.openapi.fileEditor.FileDocumentManager
import com.intellij.openapi.project.guessProjectDir
import com.intellij.openapi.ui.InputValidator
import com.intellij.openapi.ui.Messages
import com.intellij.openapi.vfs.LocalFileSystem
import com.intellij.openapi.vfs.VfsUtil
import liveplugin.PluginUtil.registerAction
import liveplugin.show
import java.io.File

val props = PropertiesComponent.getInstance()
val FILES_KEY = "sync.last_files"
val IP_KEY = "sync.last_ip"
val PORT_KEY = "sync.last_port"


val ipValidator = object : InputValidator {
    override fun checkInput(inputString: String): Boolean {
        val regex = "^(?:[0-9]{1,3}\\.){3}[0-9]{1,3}$".toRegex()
        return regex.matches(inputString)
    }
    override fun canClose(inputString: String) = checkInput(inputString)
}

val portValidator = object : InputValidator {
    override fun checkInput(inputString: String): Boolean {
        val port = inputString.toIntOrNull()
        return port != null && port in 1..65535
    }
    override fun canClose(inputString: String) = checkInput(inputString)
}

fun runTransmitter(event: AnActionEvent, ip: String, port: String, filePaths: List<String>) {
    FileDocumentManager.getInstance().saveAllDocuments()

    val project = event.project ?: return
    val projectDir = project.guessProjectDir() ?: return

    try {
        val cmd = mutableListOf("transmitter", "-i", ip, "-p", port, "-f")
        filePaths.forEach { path ->
            val file = LocalFileSystem.getInstance().findFileByPath(path)
            if (file != null) {
                VfsUtil.getRelativePath(file, projectDir, '/') ?.let { cmd.add(it) }
            }
        }

        ProcessBuilder(cmd).directory(File(project.basePath!!)).start()
        show("Syncing to $ip:$port...")
    } catch (e: Exception) {
        show("Error: ${e.message}")
    }
}

val changeConfigAction = object : AnAction("Sync: Change Config") {
    override fun actionPerformed(e: AnActionEvent) {
        val project = e.project ?: return
        val descriptor = FileChooserDescriptor(true, false, false, false, false, true).apply {
            title = "Select Files to Sync"
        }

        val files = FileChooser.chooseFiles(descriptor, project, null)
        if (files.isEmpty()) return

        val lastIp = props.getValue(IP_KEY) ?: "192.168.1.15"
        val ip = Messages.showInputDialog(project, "IP Address:", "Sync", null, lastIp, ipValidator) ?: return

        val lastPort = props.getValue(PORT_KEY) ?: "31415"
        val port = Messages.showInputDialog(project, "Port (1-65535):", "Sync", null, lastPort, portValidator) ?: return

        props.setValue(FILES_KEY, files.joinToString("|") { it.path })
        props.setValue(IP_KEY, ip)
        props.setValue(PORT_KEY, port)

        runTransmitter(e, ip, port, files.map { it.path })
    }
}

val quickSyncAction = object : AnAction("Sync: Last Config") {
    override fun actionPerformed(e: AnActionEvent) {
        val ip = props.getValue(IP_KEY)
        val port = props.getValue(PORT_KEY)
        val filesStr = props.getValue(FILES_KEY)

        if (ip == null || filesStr == null || port == null) {
            show("No saved config! Press Ctrl+Shift+S.")
            return
        }

        runTransmitter(e, ip, port, filesStr.split("|"))
    }
}

val am = ActionManager.getInstance()
listOf("SyncConfig", "SyncQuick").forEach { id -> if (am.getAction(id) != null) am.unregisterAction(id) }

registerAction("SyncConfig", "ctrl shift S", changeConfigAction)
registerAction("SyncQuick", "alt S", quickSyncAction)

show("Sync Tool Ready (Alt+S for Quick)")