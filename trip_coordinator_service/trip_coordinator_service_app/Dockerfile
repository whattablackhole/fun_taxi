FROM mcr.microsoft.com/dotnet/sdk:8.0.203 AS build

WORKDIR /app


COPY ./trip_coordinator_service.csproj ./

RUN dotnet restore

COPY . .

RUN dotnet publish -c Release -o out

FROM mcr.microsoft.com/dotnet/aspnet:8.0

WORKDIR /app

COPY --from=build /app/out .

ENTRYPOINT ["dotnet", "trip_coordinator_service.dll"]
