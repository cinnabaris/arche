CREATE TABLE todo_projects (
  id           BIGSERIAL PRIMARY KEY,
  title          VARCHAR(255)                 NOT NULL,
  description  TEXT,
  created_at   TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
  updated_at   TIMESTAMP WITHOUT TIME ZONE NOT NULL
);


CREATE TABLE todo_tasks (
  id           BIGSERIAL PRIMARY KEY,
  title          VARCHAR(255)                 NOT NULL,
  description  TEXT,
  status       VARCHAR(8) NOT NULL,
  created_at   TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
  updated_at   TIMESTAMP WITHOUT TIME ZONE NOT NULL
);
CREATE INDEX todo_tasks_status ON todo_tasks(status);

CREATE TABLE todo_logs (
  id           BIGSERIAL PRIMARY KEY,
  message      VARCHAR(500)                 NOT NULL,
  "begin" TIMESTAMP WITHOUT TIME ZONE NOT NULL,
  "end" TIMESTAMP WITHOUT TIME ZONE NOT NULL,
  task_id BIGINT NOT NULL,
  member_id BIGINT NOT NULL,
  created_at   TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now()
);
