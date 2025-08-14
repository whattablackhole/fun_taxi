async function runLoadBalancerTest() {
    let a = Array.from({ length: 100 }, () => fetchLoop());
    await Promise.all(a);
}

async function fetchLoop() {
    return fetch("http://localhost:8080/streamgate/health");
}

runLoadBalancerTest();