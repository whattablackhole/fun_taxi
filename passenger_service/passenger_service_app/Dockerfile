FROM mcr.microsoft.com/dotnet/sdk:8.0.203-alpine3.18-amd64 AS build

WORKDIR /app

COPY ./passenger_service.csproj ./

RUN dotnet restore

COPY . .

RUN dotnet publish -c Release -o out

FROM mcr.microsoft.com/dotnet/aspnet:8.0

WORKDIR /app

COPY --from=build /app/out .
COPY passenger.db .

ENTRYPOINT ["dotnet", "passenger_service.dll"]
