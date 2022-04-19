/** 人员表 */
create table if not exists `ms_core_user` (
    id bigint primary key,
    username varchar(12) comment '用户名称',
    password varchar(12) comment '密码',
    email varchar(12) comment '邮箱',
    create_at datetime comment '创建时间',
    update_at datetime comment '修改时间'
);







