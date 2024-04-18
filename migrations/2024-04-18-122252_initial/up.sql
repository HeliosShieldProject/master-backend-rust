CREATE TYPE "Country" AS ENUM ('UK', 'USA', 'Germany');

CREATE TYPE "SessionStatus" AS ENUM ('ACTIVE', 'CLOSED');

CREATE TYPE "ConfigStatus" AS ENUM ('IN_USE', 'NOT_IN_USE');

CREATE TYPE "OS" AS ENUM ('ios', 'linux', 'windows', 'macos', 'android');

CREATE TYPE "DeviceStatus" AS ENUM (
    'ACTIVE',
    'REVOKED',
    'BANNED',
    'PERMANENTLY_BANNED'
);

CREATE TYPE "UserStatus" AS ENUM ('ACTIVE', 'BANNED', 'PERMANENTLY_BANNED');

CREATE TABLE
    "Server" (
        "id" TEXT NOT NULL,
        "publicKey" TEXT NOT NULL,
        "backendUri" TEXT NOT NULL,
        "wireguardUri" TEXT NOT NULL,
        "country" "Country" NOT NULL,
        "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "updatedAt" TIMESTAMP(3) NOT NULL,
        CONSTRAINT "Server_pkey" PRIMARY KEY ("id")
    );

CREATE TABLE
    "Config" (
        "id" TEXT NOT NULL,
        "privateKey" TEXT NOT NULL,
        "userIp" TEXT NOT NULL,
        "serverId" TEXT NOT NULL,
        "status" "ConfigStatus" NOT NULL DEFAULT 'NOT_IN_USE',
        "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "updatedAt" TIMESTAMP(3) NOT NULL,
        CONSTRAINT "Config_pkey" PRIMARY KEY ("id")
    );

CREATE TABLE
    "User" (
        "id" TEXT NOT NULL,
        "email" TEXT NOT NULL,
        "password" TEXT NOT NULL,
        "bannedAt" TIMESTAMP(3),
        "bannedTill" TIMESTAMP(3),
        "status" "UserStatus" NOT NULL DEFAULT 'ACTIVE',
        "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "updatedAt" TIMESTAMP(3) NOT NULL,
        CONSTRAINT "User_pkey" PRIMARY KEY ("id")
    );

CREATE TABLE
    "Device" (
        "id" TEXT NOT NULL,
        "name" TEXT NOT NULL,
        "os" "OS" NOT NULL,
        "userId" TEXT NOT NULL,
        "bannedAt" TIMESTAMP(3),
        "bannedTill" TIMESTAMP(3),
        "revokedAt" TIMESTAMP(3),
        "status" "DeviceStatus" NOT NULL DEFAULT 'ACTIVE',
        "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "updatedAt" TIMESTAMP(3) NOT NULL,
        CONSTRAINT "Device_pkey" PRIMARY KEY ("id")
    );

CREATE TABLE
    "Session" (
        "id" TEXT NOT NULL,
        "status" "SessionStatus" NOT NULL DEFAULT 'ACTIVE',
        "openedAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "closedAt" TIMESTAMP(3),
        "deviceId" TEXT NOT NULL,
        "configId" TEXT NOT NULL,
        CONSTRAINT "Session_pkey" PRIMARY KEY ("id")
    );

CREATE UNIQUE INDEX "Server_publicKey_key" ON "Server" ("publicKey");

CREATE UNIQUE INDEX "Server_backendUri_key" ON "Server" ("backendUri");

CREATE UNIQUE INDEX "Server_wireguardUri_key" ON "Server" ("wireguardUri");

CREATE UNIQUE INDEX "Config_privateKey_key" ON "Config" ("privateKey");

CREATE UNIQUE INDEX "User_email_key" ON "User" ("email");

CREATE UNIQUE INDEX "Session_deviceId_key" ON "Session" ("deviceId");

CREATE UNIQUE INDEX "Session_configId_key" ON "Session" ("configId");

ALTER TABLE "Config" ADD CONSTRAINT "Config_serverId_fkey" FOREIGN KEY ("serverId") REFERENCES "Server" ("id") ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE "Device" ADD CONSTRAINT "Device_userId_fkey" FOREIGN KEY ("userId") REFERENCES "User" ("id") ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE "Session" ADD CONSTRAINT "Session_deviceId_fkey" FOREIGN KEY ("deviceId") REFERENCES "Device" ("id") ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE "Session" ADD CONSTRAINT "Session_configId_fkey" FOREIGN KEY ("configId") REFERENCES "Config" ("id") ON DELETE RESTRICT ON UPDATE CASCADE;