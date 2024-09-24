import { HeaderMenuComponent } from "../components/header/header-menu/HeaderMenuComponent";
import { MapComponent } from "../components/map/MapComponent";
import { useAuthService } from "../contexts/AuthServiceContext";

export function IndexView() {
  const authService = useAuthService();
  const user = authService.getUserData();
  
  return (
    <>
      <HeaderMenuComponent
        userInfo={
          <div>
            <div>{user?.username}</div>
          </div>
        }
      >
        <div className="button-link" onClick={()=>authService.addUserRole("driver")}>Become a taxi driver</div>
        <div className="button-link">Another Link</div>
      </HeaderMenuComponent>
      <MapComponent></MapComponent>
    </>
  );
}

export default IndexView;
