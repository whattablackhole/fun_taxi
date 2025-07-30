using AutoMapper;
using Microsoft.AspNetCore.Mvc;
using PassengerService.Api.DTOs;
using PassengerService.Core.Domain.Models;
using PassengerService.Core.Domain.Services;

namespace PassengerService.Api.Controllers;

[ApiController]
[Route("[controller]")]
public class TripController : ControllerBase
{
    readonly TripSearchService _tripSearchService;
    readonly IMapper _mapper;

    public TripController(TripSearchService tripSearchService, IMapper mapper)
    {
        _tripSearchService = tripSearchService;
        _mapper = mapper;
    }

    [HttpPost]
    public async Task<ActionResult> InitSearch(CreateTripSearchDTO payload)
    {
        try
        {
            var createJobModel = _mapper.Map<CreateTripSearchJob>(payload);

            await _tripSearchService.ProcessNewTripSearch(createJobModel);
            return Ok();
        }
        catch (Exception ex)
        {
            Console.WriteLine("Unhandled Exception {@ex}", ex);

            return BadRequest();
        }
    }
}
