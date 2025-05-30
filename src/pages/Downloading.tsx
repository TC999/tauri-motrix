import ContentPasteIcon from "@mui/icons-material/ContentPaste";
import FilePresentIcon from "@mui/icons-material/FilePresent";
import LinkIcon from "@mui/icons-material/Link";
import {
  Box,
  Button,
  ButtonGroup,
  SpeedDial,
  SpeedDialAction,
  SpeedDialIcon,
} from "@mui/material";
import { emit } from "@tauri-apps/api/event";
import { readText } from "@tauri-apps/plugin-clipboard-manager";
import { useLockFn } from "ahooks";
import { useRef } from "react";
import { useTranslation } from "react-i18next";

import AddTorrentDialog from "@/business/task/AddTorrentDialog";
import TaskAllAction from "@/business/task/TaskAllAction";
import TaskItem from "@/business/task/TaskItem";
import { TaskList } from "@/client/task_compose";
import { DialogRef } from "@/components/BaseDialog";
import BasePage from "@/components/BasePage";
import { Notice } from "@/components/Notice";
import { NORMAL_STATUS } from "@/constant/task";
import { ADD_DIALOG } from "@/constant/url";
import { addTaskApi } from "@/services/aria2c_api";
import { useTaskStore } from "@/store/task";

function DownloadingPage() {
  const { t } = useTranslation();

  const {
    tasks,
    selectedTaskIds,
    fetchType,
    handleTaskSelect,
    handleTaskPause,
    handleTaskResume,
    handleTaskDelete: handleTaskStop,
    openTaskFile,
    copyTaskLink,
    setFetchType,
  } = useTaskStore();

  const torrentRef = useRef<DialogRef>(null);

  const addTaskByClipboard = useLockFn(async () => {
    try {
      const content = await readText();
      await addTaskApi(content, {});
    } catch (e) {
      // @ts-expect-error string or any
      Notice.error(e.message ?? e);
    }
  });

  return (
    <BasePage
      title={t("Task-Start")}
      header={
        <Box sx={{ display: "flex", gap: 1, alignItems: "center" }}>
          <TaskAllAction
            onPause={handleTaskPause}
            onResume={handleTaskResume}
            onStop={handleTaskStop}
            selectedTaskIds={selectedTaskIds}
            fetchType={fetchType}
          />
          <ButtonGroup size="small">
            {NORMAL_STATUS.map((value) => (
              <Button
                key={value}
                variant={value === fetchType ? "contained" : "outlined"}
                onClick={() => setFetchType(value)}
                sx={{ textTransform: "capitalize" }}
              >
                {t(`Button-Fetch-Type.${value}`)}
              </Button>
            ))}
          </ButtonGroup>
        </Box>
      }
      fab={
        <SpeedDial
          ariaLabel="add task fab"
          sx={{ position: "absolute", bottom: 16, right: 16 }}
          icon={<SpeedDialIcon />}
        >
          <SpeedDialAction
            icon={<LinkIcon />}
            onClick={() => emit(ADD_DIALOG)}
            title={t("common.FromUrl")}
          />
          <SpeedDialAction
            icon={<FilePresentIcon />}
            title={t("common.FromTorrentFile")}
            onClick={() => torrentRef.current?.open()}
          />
          <SpeedDialAction
            icon={<ContentPasteIcon />}
            title={t("common.FromClipboard")}
            onClick={addTaskByClipboard}
          />
        </SpeedDial>
      }
    >
      <TaskList
        dataSource={tasks}
        renderItem={(task) => (
          <TaskItem
            onCopyLink={copyTaskLink}
            onStop={handleTaskStop}
            onResume={handleTaskResume}
            onPause={handleTaskPause}
            onOpenFile={openTaskFile}
            key={task.gid}
            task={task}
            onSelect={handleTaskSelect}
            selected={selectedTaskIds.includes(task.gid)}
          />
        )}
      />
      <AddTorrentDialog ref={torrentRef} />
    </BasePage>
  );
}

export default DownloadingPage;
