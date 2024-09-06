CREATE TABLE public."user" (
	id uuid NOT NULL DEFAULT gen_random_uuid(),
	phone_number varchar NOT NULL,
	"name" varchar NOT NULL,
	pass_hash varchar NOT NULL,
	address varchar NOT NULL,
	"role" smallint NOT NULL DEFAULT 1,
	CONSTRAINT user_pk PRIMARY KEY (id),
	CONSTRAINT user_un_phone_number UNIQUE (phone_number)
);

CREATE TABLE public.user_token (
	id char(32) NOT NULL,
	user_id uuid NOT NULL,
	CONSTRAINT user_token_pk PRIMARY KEY (id),
	CONSTRAINT user_token_fk_user FOREIGN KEY (user_id) REFERENCES public."user"(id)
);

CREATE TABLE public."order" (
	id uuid NOT NULL,
	"date" timestamptz NOT NULL DEFAULT now(),
	amount smallint NOT NULL,
	"state" smallint NOT NULL DEFAULT 1,
	user_id uuid NOT NULL,
	attachment_extension varchar NOT NULL,
	CONSTRAINT order_pk PRIMARY KEY (id),
	CONSTRAINT order_fk_user FOREIGN KEY (user_id) REFERENCES public."user"(id)
);
