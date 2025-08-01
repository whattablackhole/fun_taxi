using MassTransit;
using MassTransit.EntityFrameworkCoreIntegration;
using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Metadata.Builders;

public class TripSagaDbContext : SagaDbContext
{
    public TripSagaDbContext(DbContextOptions<TripSagaDbContext> options)
        : base(options) { }

    public DbSet<TripState> Orders { get; set; }

    protected override IEnumerable<ISagaClassMap> Configurations
    {
        get { yield return new TripStateMap(); }
    }
}

public class TripStateMap : SagaClassMap<TripState>
{
    protected override void Configure(EntityTypeBuilder<TripState> entity, ModelBuilder model)
    {
        entity.Property(x => x.CurrentState).HasMaxLength(64);
    }
}
