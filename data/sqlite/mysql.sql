CREATE TABLE `sys_menu` (
	`menu_id` BIGINT(19) NOT NULL AUTO_INCREMENT COMMENT '菜单ID',
	`parent_id` BIGINT(19) NULL DEFAULT '0' COMMENT '父菜单ID',
	`menu_name` VARCHAR(50) NOT NULL COMMENT '菜单名称' COLLATE 'utf8_general_ci',
	`path` VARCHAR(200) NULL DEFAULT '' COMMENT '路由地址' COLLATE 'utf8_general_ci',
	`component` VARCHAR(255) NULL DEFAULT NULL COMMENT '组件路径' COLLATE 'utf8_general_ci',
	`query` VARCHAR(255) NULL DEFAULT NULL COMMENT '路由参数' COLLATE 'utf8_general_ci',
	`is_frame` INT(10) NULL DEFAULT '1' COMMENT '是否为外链（0是 1否）',
	`is_cache` INT(10) NULL DEFAULT '0' COMMENT '是否缓存（0缓存 1不缓存）',
	`menu_type` CHAR(1) NULL DEFAULT '' COMMENT '菜单类型（M目录 C菜单 F按钮）' COLLATE 'utf8_general_ci',
	`visible` CHAR(1) NULL DEFAULT '0' COMMENT '菜单状态（0显示 1隐藏）' COLLATE 'utf8_general_ci',
	`status` CHAR(1) NULL DEFAULT '0' COMMENT '菜单状态（0正常 1停用）' COLLATE 'utf8_general_ci',
	`perms` VARCHAR(100) NULL DEFAULT NULL COMMENT '权限标识' COLLATE 'utf8_general_ci',
	`icon` VARCHAR(100) NULL DEFAULT '#' COMMENT '菜单图标' COLLATE 'utf8_general_ci',
	`remark` VARCHAR(500) NULL DEFAULT '' COMMENT '备注' COLLATE 'utf8_general_ci',
	`order_num` INT(10) NULL DEFAULT '0' COMMENT '显示顺序',
	`create_by` VARCHAR(64) NULL DEFAULT '' COMMENT '创建者' COLLATE 'utf8_general_ci',
	`create_time` DATETIME NULL DEFAULT NULL COMMENT '创建时间',
	`update_by` VARCHAR(64) NULL DEFAULT '' COMMENT '更新者' COLLATE 'utf8_general_ci',
	`update_time` DATETIME NULL DEFAULT NULL COMMENT '更新时间',
	PRIMARY KEY (`menu_id`) USING BTREE
)
COMMENT='菜单权限表'
COLLATE='utf8_general_ci'
ENGINE=InnoDB
AUTO_INCREMENT=2007
;

CREATE TABLE `sys_role` (
	`role_id` BIGINT(19) NOT NULL AUTO_INCREMENT COMMENT '角色ID',
	`role_name` VARCHAR(30) NOT NULL COMMENT '角色名称' COLLATE 'utf8_general_ci',
	`role_key` VARCHAR(100) NOT NULL COMMENT '角色权限字符串' COLLATE 'utf8_general_ci',
	`role_sort` INT(10) NOT NULL COMMENT '显示顺序',
	`data_scope` CHAR(1) NULL DEFAULT '1' COMMENT '数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）' COLLATE 'utf8_general_ci',
	`menu_check_strictly` TINYINT(1) NULL DEFAULT '1' COMMENT '菜单树选择项是否关联显示',
	`dept_check_strictly` TINYINT(1) NULL DEFAULT '1' COMMENT '部门树选择项是否关联显示',
	`status` CHAR(1) NOT NULL COMMENT '角色状态（0正常 1停用）' COLLATE 'utf8_general_ci',
	`del_flag` CHAR(1) NULL DEFAULT '0' COMMENT '删除标志（0代表存在 2代表删除）' COLLATE 'utf8_general_ci',
	`remark` VARCHAR(500) NULL DEFAULT NULL COMMENT '备注' COLLATE 'utf8_general_ci',
	`create_by` VARCHAR(64) NULL DEFAULT '' COMMENT '创建者' COLLATE 'utf8_general_ci',
	`create_time` DATETIME NULL DEFAULT NULL COMMENT '创建时间',
	`update_by` VARCHAR(64) NULL DEFAULT '' COMMENT '更新者' COLLATE 'utf8_general_ci',
	`update_time` DATETIME NULL DEFAULT NULL COMMENT '更新时间',
	PRIMARY KEY (`role_id`) USING BTREE
)
COMMENT='角色信息表'
COLLATE='utf8_general_ci'
ENGINE=InnoDB
AUTO_INCREMENT=100
;

CREATE TABLE `sys_role_menu` (
	`role_id` BIGINT(19) NOT NULL COMMENT '角色ID',
	`menu_id` BIGINT(19) NOT NULL COMMENT '菜单ID',
	PRIMARY KEY (`role_id`, `menu_id`) USING BTREE
)
COMMENT='角色和菜单关联表'
COLLATE='utf8_general_ci'
ENGINE=InnoDB
;

CREATE TABLE `sys_user` (
	`user_id` BIGINT(19) NOT NULL AUTO_INCREMENT COMMENT '用户ID',
	`user_name` VARCHAR(30) NOT NULL COMMENT '用户账号' COLLATE 'utf8_general_ci',
	`nick_name` VARCHAR(30) NOT NULL COMMENT '用户昵称' COLLATE 'utf8_general_ci',
	`user_type` VARCHAR(2) NULL DEFAULT '00' COMMENT '用户类型（00系统用户）' COLLATE 'utf8_general_ci',
	`dept_id` BIGINT(19) NULL DEFAULT NULL COMMENT '部门ID',
	`email` VARCHAR(50) NULL DEFAULT '' COMMENT '用户邮箱' COLLATE 'utf8_general_ci',
	`phonenumber` VARCHAR(11) NULL DEFAULT '' COMMENT '手机号码' COLLATE 'utf8_general_ci',
	`sex` CHAR(1) NULL DEFAULT '0' COMMENT '用户性别（0男 1女 2未知）' COLLATE 'utf8_general_ci',
	`avatar` VARCHAR(100) NULL DEFAULT '' COMMENT '头像地址' COLLATE 'utf8_general_ci',
	`password` VARCHAR(100) NULL DEFAULT '' COMMENT '密码' COLLATE 'utf8_general_ci',
	`status` CHAR(1) NULL DEFAULT '0' COMMENT '帐号状态（0正常 1停用）' COLLATE 'utf8_general_ci',
	`del_flag` CHAR(1) NULL DEFAULT '0' COMMENT '删除标志（0代表存在 2代表删除）' COLLATE 'utf8_general_ci',
	`login_ip` VARCHAR(128) NULL DEFAULT '' COMMENT '最后登录IP' COLLATE 'utf8_general_ci',
	`login_date` DATETIME NULL DEFAULT NULL COMMENT '最后登录时间',
	`remark` VARCHAR(500) NULL DEFAULT NULL COMMENT '备注' COLLATE 'utf8_general_ci',
	`scode` VARCHAR(20) NULL DEFAULT NULL COMMENT '学校编号' COLLATE 'utf8_general_ci',
	`create_by` VARCHAR(64) NULL DEFAULT '' COMMENT '创建者' COLLATE 'utf8_general_ci',
	`create_time` DATETIME NULL DEFAULT NULL COMMENT '创建时间',
	`update_by` VARCHAR(64) NULL DEFAULT '' COMMENT '更新者' COLLATE 'utf8_general_ci',
	`update_time` DATETIME NULL DEFAULT NULL COMMENT '更新时间',
	PRIMARY KEY (`user_id`) USING BTREE
)
COMMENT='用户信息表'
COLLATE='utf8_general_ci'
ENGINE=InnoDB
AUTO_INCREMENT=100
;

CREATE TABLE `sys_user_role` (
	`user_id` BIGINT(19) NOT NULL COMMENT '用户ID',
	`role_id` BIGINT(19) NOT NULL COMMENT '角色ID',
	PRIMARY KEY (`user_id`, `role_id`) USING BTREE
)
COMMENT='用户和角色关联表'
COLLATE='utf8_general_ci'
ENGINE=InnoDB
;

