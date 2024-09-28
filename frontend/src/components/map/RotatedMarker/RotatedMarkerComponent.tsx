// @ts-nocheck
import L from 'leaflet';
import { useMap } from 'react-leaflet';
import React from 'react';

const LeafletMarker = L.Marker.extend({
  _setPos(pos) {
    L.Marker.prototype._setPos.call(this, pos); // Ensure calling the original method
    this._setRotation(this.options.rotation);
  },
  _setRotation(rotation) {
    if (typeof rotation === 'number' && this._icon) { // Check if _icon is available
      this._icon.style[L.DomUtil.TRANSFORM + 'Origin'] = this.options.rotationOrigin || 'center';
      const transform = this._icon.style[L.DomUtil.TRANSFORM] + ` rotate(${rotation}deg)`;
      this._icon.style[L.DomUtil.TRANSFORM] = transform;
    }
  },
});

const createRotatedMarker = (position, options) => {
  return new LeafletMarker(position, options);
};

const RotatedMarker = (props) => {
  const map = useMap();

  React.useEffect(() => {
    const marker = createRotatedMarker(props.position, { ...props });

    marker.addTo(map);

    return () => {
      map.removeLayer(marker);
    };
  }, [map, props.position, props.rotation, props.rotationOrigin]); // Add dependencies here

  return null; // Since we're manually handling the marker, this component doesn't render anything itself
};

export default RotatedMarker;