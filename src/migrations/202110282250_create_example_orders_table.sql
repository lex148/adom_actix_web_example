

CREATE TABLE public.orders
(
    id serial NOT NULL,
    customer text not null,
    total bigint not null
);

insert into orders (
    customer, total 
)
select
    md5(random()::text),
    floor(random() * 1000000 + 1)::int
from generate_series(1, 100000);
