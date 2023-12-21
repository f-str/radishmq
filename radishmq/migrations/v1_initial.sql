create extension if not exists "uuid-ossp";

create table message_topic if not exists (
                       id uuid default uuid_generate_v4(),
                       topic_name varchar(255) not null unique,
                       data_index int not null default 0,
                       constraint message_topic_pkey primary key (id)
);

create table message_topic_subscriber if not exists (
                            id uuid default uuid_generate_v4(),
                            subscriber_name varchar(255) not null unique,
                            message_topic_id int not null,
                            subscriber_index int not null default 0,
                            constraint message_topic_subscriber_pkey primary key (id),
                            constraint message_topic_subscriber_topic_id_fkey foreign key (message_topic_id) references message_topic(id)
);

create table message_topic_publisher if not exists (
                            id uuid default uuid_generate_v4(),
                            publisher_name varchar(255) not null unique,
                            message_topic_id int not null,
                            constraint message_topic_publisher_pkey primary key (id),
                            constraint message_topic_publisher_topic_id_fkey foreign key (message_topic_id) references message_topic(id)
);

create table task_topic if not exists (
                       id uuid default uuid_generate_v4(),
                       topic_name varchar(255) not null unique,
                       constraint task_topic_pkey primary key (id)
);

create table task_topic_subscriber if not exists (
                            id uuid default uuid_generate_v4(),
                            subscriber_name varchar(255) not null unique,
                            task_topic_id int not null,
                            constraint task_topic_subscriber_pkey primary key (id),
                            constraint task_topic_subscriber_topic_id_fkey foreign key (task_topic_id) references task_topic(id)
);

create table task_topic_publisher if not exists (
                            id uuid default uuid_generate_v4(),
                            publisher_name varchar(255) not null unique,
                            task_topic_id int not null,
                            constraint task_topic_publisher_pkey primary key (id),
                            constraint task_topic_publisher_topic_id_fkey foreign key (task_topic_id) references task_topic(id)
);
