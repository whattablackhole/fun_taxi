using AutoMapper;
using PassengerService.Api.DTOs;
using PassengerService.Core.Domain.Models;
using PassengerService.Infrastructure.Data.Models;
using PassengerService.Infrastructure.MessageBrockers.Models;

namespace PassengerService.Mapping;

public class MappingProfile : Profile
{
    public MappingProfile()
    {
        CreateMap<LocationDTO, GeoLocation>();
        CreateMap<CreateTripSearchDTO, CreateTripSearchJob>();
        CreateMap<TripJob, TripSearchRequestedMessage>();
    }
}
