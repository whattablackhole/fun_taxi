import { useMemo } from "react";
import { useAuthService } from "../../contexts/AuthServiceContext";
import { RoleName } from "../../models/UserModels";

export function RoleBasedComponent({
  children,
  exclude,
  include,
}: {
  children: React.ReactNode;
  exclude?: RoleName[];
  include?: RoleName[];
}) {
  const { authService, userData } = useAuthService();

  const shouldRender = useMemo(() => {
    const shouldRender =
      (exclude?.some((r) => {
        return !authService.checkUserRole(r);
      }) ??
        true) &&
      (include?.every((r) => {
        return authService.checkUserRole(r);
      }) ??
        true);
    return shouldRender;
  }, [userData?.roles, exclude, include]);

  if (!shouldRender) {
    return null;
  }

  return <>{children}</>;
}
