// OAP/dashboard_ui/globe_3d_render.js
const globeContainer = document.getElementById('globe-container');
const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(40, globeContainer.clientWidth / globeContainer.clientHeight, 0.1, 1000);
const renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true });
renderer.setSize(globeContainer.clientWidth, globeContainer.clientHeight);
renderer.setPixelRatio(window.devicePixelRatio);
globeContainer.appendChild(renderer.domElement);

// Group Objects for easy rotation together
const earthGroup = new THREE.Group();
scene.add(earthGroup);

// Black Earth Core
const coreGeom = new THREE.SphereGeometry(4.9, 32, 32);
const coreMat = new THREE.MeshBasicMaterial({ color: 0x02050a });
earthGroup.add(new THREE.Mesh(coreGeom, coreMat));

// Wireframe Earth Skin (Thin Cyan)
const earthGeom = new THREE.SphereGeometry(5, 32, 32);
const earthMat = new THREE.MeshBasicMaterial({ color: 0x00f3ff, wireframe: true, transparent: true, opacity: 0.15 });
earthGroup.add(new THREE.Mesh(earthGeom, earthMat));

// Orbital Rings (Outer & Inner)
function createOrbit(radius, opacity) {
    const geo = new THREE.RingGeometry(radius, radius + 0.02, 64);
    const mat = new THREE.MeshBasicMaterial({ color: 0x00f3ff, side: THREE.DoubleSide, transparent: true, opacity: opacity });
    const mesh = new THREE.Mesh(geo, mat);
    mesh.rotation.x = Math.PI / 2;
    return mesh;
}
scene.add(createOrbit(6.5, 0.3)); // Main orbit
scene.add(createOrbit(8.0, 0.1)); // Wide sensor orbit

// Satellite
const satGeom = new THREE.SphereGeometry(0.15, 8, 8);
const satMat = new THREE.MeshBasicMaterial({ color: 0xffffff });
const satellite = new THREE.Mesh(satGeom, satMat);
scene.add(satellite);

camera.position.set(0, 8, 18);
camera.lookAt(0, 0, 0);

// Animation
let theta = 0;
function animateGlobe() {
    requestAnimationFrame(animateGlobe);
    earthGroup.rotation.y += 0.002;
    
    // The satellite orbits the main orbit (radius 6.5)
    theta += 0.015;
    satellite.position.x = 6.5 * Math.cos(theta);
    satellite.position.z = 6.5 * Math.sin(theta);
    
    renderer.render(scene, camera);
}
animateGlobe();

// Automatic Telemetry Generator
const logElement = document.getElementById('telemetry-log');
function addLog() {
    const isCritical = Math.random() > 0.95;
    const time = new Date().toISOString().split('T')[1].split('.')[0];
    const log = document.createElement('div');
    log.className = isCritical ? 'log-entry log-critical' : 'log-entry';
    
    if (isCritical) {
        log.innerHTML = `<strong>[${time}]</strong> [WARNING] COLLISION PROBABILITY > 80%. TCA: 1.2s.`;
    } else {
        const x = satellite.position.x.toFixed(2);
        const z = satellite.position.z.toFixed(2);
        log.innerHTML = `<strong>[${time}]</strong> NAV-SYS: Pos(X:${x}, Z:${z}) | J2 Perturbation: COMPENSATED`;
    }
    
    logElement.prepend(log);
    if (logElement.children.length > 20) logElement.removeChild(logElement.lastChild);
}
setInterval(addLog, 1500);

// 3D Camera Auto-Resize
window.addEventListener('resize', () => {
    camera.aspect = globeContainer.clientWidth / globeContainer.clientHeight;
    camera.updateProjectionMatrix();
    renderer.setSize(globeContainer.clientWidth, globeContainer.clientHeight);
});

// Mission Hour Update
setInterval(() => {
    const d = new Date();
    document.getElementById('mission-time').innerText = `T+ 00:${d.getMinutes()}:${d.getSeconds()}`;
}, 1000);