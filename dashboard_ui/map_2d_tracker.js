// OAP/dashboard_ui/map_2d_tracker.js
const canvas = document.getElementById('map-2d');
const ctx = canvas.getContext('2d');
const container = document.getElementById('map-container');

function resizeCanvas() {
    canvas.width = container.clientWidth;
    canvas.height = container.clientHeight;
}
window.addEventListener('resize', resizeCanvas);
resizeCanvas();

let satLon = 0;
let radarAngle = 0;

function drawPremiumMap() {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    
    ctx.strokeStyle = 'rgba(255, 255, 255, 0.05)';
    ctx.lineWidth = 1;
    for(let i = 0; i <= canvas.width; i += canvas.width/12) { ctx.beginPath(); ctx.moveTo(i, 0); ctx.lineTo(i, canvas.height); ctx.stroke(); }
    for(let i = 0; i <= canvas.height; i += canvas.height/6) { ctx.beginPath(); ctx.moveTo(0, i); ctx.lineTo(canvas.width, i); ctx.stroke(); }

    const equatorY = canvas.height / 2;
    ctx.strokeStyle = 'rgba(0, 243, 255, 0.3)';
    ctx.beginPath(); ctx.moveTo(0, equatorY); ctx.lineTo(canvas.width, equatorY); ctx.stroke();

    // Satellite Position (Sinusoidal Path)
    satLon += 0.5;
    if(satLon > canvas.width) satLon = 0;
    const satLat = equatorY + Math.sin(satLon * 0.05) * 30; // Slightly inclined orbit simulation

    // Radar Sweep Image
    radarAngle += 0.05;
    ctx.beginPath();
    ctx.moveTo(satLon, satLat);
    ctx.arc(satLon, satLat, 40, radarAngle, radarAngle + 0.5);
    ctx.lineTo(satLon, satLat);
    ctx.fillStyle = 'rgba(0, 243, 255, 0.15)';
    ctx.fill();

    // Satellite Images
    ctx.fillStyle = '#ffffff';
    ctx.shadowBlur = 10;
    ctx.shadowColor = '#00f3ff';
    ctx.beginPath(); ctx.arc(satLon, satLat, 3, 0, Math.PI * 2); ctx.fill();
    ctx.shadowBlur = 0; // Reset

    requestAnimationFrame(drawPremiumMap);
}

drawPremiumMap();