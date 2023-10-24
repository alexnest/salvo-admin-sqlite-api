DROP TABLE IF EXISTS  sys_menu;
create table sys_menu
(
    id          TEXT
        constraint sys_menu_pk
            primary key,
    pid         TEXT,
    level       TEXT,
    name        TEXT,
    path        TEXT,
    component   TEXT,
    query       TEXT,
    is_frame    INTEGER,
    is_cache    INTEGER,
    type        TEXT,
    visible     INTEGER,
    status      INTEGER,
    perms       TEXT,
    icon        TEXT,
    remark      TEXT,
    order_num   INTEGER,
    create_by   TEXT,
    create_time TEXT,
    update_by   TEXT,
    update_time TEXT
);

DROP TABLE IF EXISTS sys_role;
create table sys_role
(
    id              TEXT
                    constraint sys_role_pk
                    primary key,
    name            TEXT,
    key             TEXT,
    sort            INTEGER,
    scope           INTEGER,
    menu_strictly   INTEGER,
    dept_strictly   INTEGER,
    status          INTEGER,
    del_flag        INTEGER,
    remark          TEXT,
    create_by       TEXT,
    create_time     TEXT,
    update_by       TEXT,
    update_time     TEXT
);

DROP TABLE IF EXISTS sys_role_menu;
create table sys_role_menu
(
    id              TEXT
                    constraint sys_role_menu_pk
                    primary key,
    role_id         TEXT,
    menu_id         TEXT
);

DROP TABLE IF EXISTS sys_user;
create table sys_user 
(
    id            TEXT
                  constraint sys_user_pk
                  primary key,
    name          TEXT,
    nick_name     TEXT,
    type          INTEGER,
    dept_id       TEXT,
    email         TEXT,
    phonenumber   TEXT,
    sex           INTEGER,
    avatar        TEXT,
    password      TEXT,
    status        INTEGER,
    del_flag      INTEGER,
    login_ip      TEXT,
    login_date    TEXT,
    remark        TEXT,
    scode         TEXT,
    create_by     TEXT,
    create_time   TEXT,
    update_by     TEXT,
    update_time   TEXT
);

DROP TABLE IF EXISTS sys_user_role;
create table sys_user_role
(
    id              TEXT
                    constraint sys_user_role_pk
                    primary key,
    user_id         TEXT,
    role_id         TEXT
);

