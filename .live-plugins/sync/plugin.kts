import com.intellij.openapi.actionSystem.ActionManager
import com.intellij.openapi.actionSystem.AnAction
import com.intellij.openapi.actionSystem.AnActionEvent
import com.intellij.openapi.fileChooser.FileChooser
import com.intellij.openapi.fileChooser.FileChooserDescriptor
import com.intellij.openapi.project.guessProjectDir
import com.intellij.openapi.ui.Messages
import com.intellij.openapi.vfs.VfsUtil
import com.intellij.openapi.vfs.VirtualFile
import liveplugin.registerAction
import liveplugin.show
import java.io.File


val actionId = "SyncAction"
val actionManager: ActionManager = ActionManager.getInstance()

if (actionManager.getAction(actionId) != null) {
    actionManager.unregisterAction(actionId)
}

val syncAction = object : AnAction("Sync", "Pick files relative to project root", null) {
    override fun actionPerformed(event: AnActionEvent) {
        val project = event.project ?: return
        val projectDir: VirtualFile = project.guessProjectDir() ?: return


        val descriptor = FileChooserDescriptor(true, false, false, false, false, true)
        descriptor.title = "Select Files to Sync"

        val files = FileChooser.chooseFiles(descriptor, project, null)
        if (files.isEmpty()) return

        val ip = Messages.showInputDialog(
            project,
            "Target IP:",
            "Transmitter",
            null,
            "192.168.1.15",
            null
        ) ?: return

        val port = Messages.showInputDialog(
            project,
            "Target port:",
            "Transmitter",
            null,
            "31415",
            null
        ) ?: return


        try {
            val cmd = mutableListOf("transmitter", "-i", ip, "-p", port, "-f")

            // Convert absolute paths to project-relative paths
            files.forEach { file ->
                val relativePath = VfsUtil.getRelativePath(file, projectDir, '/')
                if (relativePath != null) {
                    cmd.add(relativePath)
                }
            }

            ProcessBuilder(cmd)
                .directory(File(project.basePath!!))
                .start()

            show("Syncing relative paths: ${cmd.drop(4).joinToString(", ")}")
        } catch (e: Exception) {
            show("Error: ${e.message}")
        }
    }
}

registerAction(id = actionId, keyStroke = "ctrl shift S", action = syncAction)
show("Relative Path Sync Loaded")