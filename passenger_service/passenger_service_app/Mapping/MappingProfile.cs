using AutoMapper;
using FunTaxi.Messages.Trips.V1;
using PassengerService.Api.DTOs;
using PassengerService.Core.Domain.Models;
using PassengerService.Infrastructure.Data.Models;

namespace PassengerService.Mapping;

public class MappingProfile : Profile
{
    public MappingProfile()
    {
        CreateMap<LocationDTO, GeoLocation>();
        CreateMap<CreateTripSearchDTO, CreateTripSearchJob>();
        CreateMap<TripJob, PassengerTripRequested>();
    }
}
