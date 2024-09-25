import { HeaderMenuComponent } from "../components/header/header-menu/HeaderMenuComponent";
import { MapComponent } from "../components/map/MapComponent";
import { RoleBasedComponent } from "../components/shared/RoleBasedComponent";
import { useAuthService } from "../contexts/AuthServiceContext";
import { Order } from "../models/OrderModels";

export function IndexView() {
  const { authService, userData } = useAuthService();

  const onOrderApply = async (order: Order)=> {
    const result = await fetch("http://localhost:8000/submit_order/", {
      method: "POST",
      headers: {
        // Authorization: `Bearer ${this.accessToken}`,
        "Content-Type": "application/json",
      },
      body: JSON.stringify(order)
    });


  }

  return (
    <>
      <HeaderMenuComponent
        userInfo={
          <div>
            <div>{userData?.username}</div>
          </div>
        }
      >
        <RoleBasedComponent exclude={["driver"]}>
          <div
            className="button-link"
            onClick={async () => authService.addUserToDriversGroup()}
          >
            Become a taxi driver
          </div>
        </RoleBasedComponent>
        <div className="button-link">Another Link</div>
      </HeaderMenuComponent>
      <MapComponent handlers={{onOrderApply}}></MapComponent>
    </>
  );
}

export default IndexView;
