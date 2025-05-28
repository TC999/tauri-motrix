import { Divider } from "@mui/material";
import { useTranslation } from "react-i18next";

import { TaskDrawerItem, TaskDrawerList } from "@/client/task_compose";
import { Aria2Task } from "@/services/aria2c_api";
import { getTaskName } from "@/utils/task";

export interface TaskInfoPanelProps {
  task: Aria2Task;
}

function TaskInfoPanel({ task }: TaskInfoPanelProps) {
  const { t } = useTranslation();
  const taskName = getTaskName(task);

  return (
    <TaskDrawerList title={t("task.InfoDetails")}>
      <TaskDrawerItem label="GID" value={task.gid} />
      <Divider />
      <TaskDrawerItem label={t("task.TaskName")} value={taskName} />
      <Divider />
      <TaskDrawerItem label={t("task.SaveTo")} value={task.dir} />
      <Divider />
      <TaskDrawerItem label={t("task.Status")} value={task.status} />
    </TaskDrawerList>
  );
}

export default TaskInfoPanel;
