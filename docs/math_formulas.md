# Mathematical Foundation for OAP

This document outlines the astrodynamics, physics models, and spatial geometry algorithms implemented within the OAP core engine.

## 1. Orbital Propagation (Two-Body Problem)
Satellite state vectors are propagated using Newton's law of universal gravitation. The acceleration vector $\mathbf{a}$ experienced by an object at a position vector $\mathbf{r}$ relative to the Earth's center of mass is defined as:

$$\mathbf{a} = -\frac{\mu}{|\mathbf{r}|^3} \mathbf{r}$$

Where:
* $\mu$: Earth's standard gravitational parameter ($3.986004418 \times 10^{14} \text{ m}^3/\text{s}^2$).
* $\mathbf{r}$: Position vector relative to the Earth's geocenter.

## 2. J2 Perturbation (Earth Oblateness)
To maintain deterministic precision for Low Earth Orbit (LEO) assets, OAP accounts for the Earth's non-spherical mass distribution. The $J_2$ perturbing acceleration corrects secular variations in the orbital elements.

$$\mathbf{a}_{J2} = \frac{3}{2} J_2 \left(\frac{\mu}{r^2}\right) \left(\frac{R_e}{r}\right)^2 \begin{bmatrix} (5\frac{z^2}{r^2} - 1)\frac{x}{r} \\ (5\frac{z^2}{r^2} - 1)\frac{y}{r} \\ (5\frac{z^2}{r^2} - 3)\frac{z}{r} \end{bmatrix}$$

Where:
* $J_2$: Earth's second zonal harmonic constant ($1.08262668 \times 10^{-3}$).
* $R_e$: Earth's mean equatorial radius ($6378137.0 \text{ m}$).
* $x, y, z$: Geocentric equatorial coordinates of the satellite.
* $r$: Magnitude of the position vector.

## 3. Time of Closest Approach (TCA)
Conjunction assessment is executed by evaluating the relative distance between two orbital objects. TCA is derived by finding the minimum of the relative distance function using the dot product of relative position ($\mathbf{r}_{rel}$) and relative velocity ($\mathbf{v}_{rel}$):

$$t_{tca} = -\frac{\mathbf{r}_{rel} \cdot \mathbf{v}_{rel}}{|\mathbf{v}_{rel}|^2}$$

The system classifies a threat as critical if $t_{tca} > 0$ and the projected scalar distance at $t_{tca}$ falls below the hard-deck safety threshold.

## 4. Evasive Maneuver Vectorization
To maximize Delta-V efficiency, the evasion thrust vector ($\mathbf{n}$) is calculated strictly normal to the orbital plane using the cross product of the current position and velocity vectors:

$$\mathbf{n} = \frac{\mathbf{r} \times \mathbf{v}}{|\mathbf{r} \times \mathbf{v}|}$$

This ensures the applied thrust alters the orbital trajectory without drastically affecting the semi-major axis or orbital velocity profile.