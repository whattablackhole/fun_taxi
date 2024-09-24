import React from "react";
import { LogoComponent } from "../../shared/LogoComponent";
import "./HeaderMenuComponent.scss";

interface HeaderMenuComponentProps {
  children: React.ReactNode;
  userInfo: React.ReactNode;
}

export function HeaderMenuComponent({ children, userInfo }: HeaderMenuComponentProps) {
  return (
    <>
      <div className="header-menu__container">
        <LogoComponent />
        <div className="header-menu__list">
          {React.Children.map(children, (child) => (
            <div className="header-menu__item">
              {child}
            </div>
          ))}
        </div>
        <div className="header-menu__user-info">
          {userInfo}
        </div>
      </div>
    </>
  );
}