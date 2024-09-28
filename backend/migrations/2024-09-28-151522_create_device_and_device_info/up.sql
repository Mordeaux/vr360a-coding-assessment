-- Your SQL goes here
CREATE TABLE "device"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"hostname" VARCHAR NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP NOT NULL
);

CREATE TABLE "device_info"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"total_memory" INT8 NOT NULL,
	"used_memory" INT8 NOT NULL,
	"total_swap" INT8 NOT NULL,
	"used_swap" INT8 NOT NULL,
	"system_name" VARCHAR NOT NULL,
	"kernel_version" VARCHAR NOT NULL,
	"os_version" VARCHAR NOT NULL,
	"hostname" VARCHAR NOT NULL,
	"number_of_cpus" INT4 NOT NULL,
	"timestamp" INT8 NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP NOT NULL,
	"device_id" INT4 NOT NULL,
	FOREIGN KEY ("device_id") REFERENCES "device"("id")
);

