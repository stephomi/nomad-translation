// german translation by djblueprint / www.3d-board.de
// comments with "ICON FIT" means < 10 characters
// arguments with $0 $1 etc

// general stuffs

// Popup question, confirm? [yes/cancel] [ok/cancel] [delete/cancel]
confirm "Bestätigen?"
yes "Ja"
ok "OK"
cancel "Abbrechen"
delete "Löschen"

// feature: Auto / Off / On
on "An"
off "Aus"
auto "Auto"

// coordinate
X "X"
Y "Y"
Z "Z"

advancedSettings "Erweitert"
noSelectedMesh "Kein Mesh ausgewählt"

// generic warning (typically in menu like layer or material)
multipleObjectWarning "Mehrere Meshes sind ausgewählt, bitte nur ein Mesh auswählen."

// --------------------------------------------------------------------------------------

// when you launch the app and there is missing Nomad/data files
loading.reprocess "Fehlende Vorschaubilder, erstelle neu... ($0/$1)

$2"

// main pbr channel
baseColor "Color"
roughness "Roughness"
metalness "Metalness"

// --------------------------------------------------------------------------------------
// about
about.minify "Menüs ausblenden"
about.minify.help "Sie können auch mit 4 Fingern auf den Bildschirm tippen, wenn Ihr Gerät dies unterstützt."
about.turntable "Turntable"
about.turntableSpeed "Geschwindigkeit Turntable"
about.credits "Credits"
about.creditsOpenSource "Open-Source"
about.creditsArts "MatCaps & HDRIs"
about.languages "Sprachen"
about.languages.help "Die Übersetzung ist verfügbar unter github.com/stephomi/nomad-translation"
about.openUrl "Öffnen?"
// nomad
about.website "Webseite"
about.forum "Forum"
about.manual "Handbuch"
about.mail "Support"
// social
about.twitter "Twitter"
about.instagram "Instagram"
about.facebook "Facebook"
about.discord "Discord"

// --------------------------------------------------------------------------------------
// alert
alert.hole.nothing "Das Objekt hat keine Löcher!"
alert.shape.notVisible "Das aktuelle Mesh ist unsichtbar!"
alert.trim.nothing "Nichts zu beschneiden."
alert.trim.full  "Beschneiden abgebrochen: Das Mesh würde vollständig beschnitten werden."
alert.mask.noExtract "Nichts zum Extrahieren!"
alert.mask.noSplit "Nichts zum Teilen ausgewählt!"
alert.view.disabled "Funktionen im Ansichtsmodus deaktiviert:"
alert.view.disabled.widgetPrimitive "Primitive widgets"
alert.separate.fail "Kann nicht getrennt werden: Das Objekt hat nur einen Teil!"
alert.voxelRemesh.success "Remeshing ausgeführt!"
alert.voxelRemesh.empty "Remeshing abgebrochen: Das Ergebnis hätte keine Faces mehr."
alert.voxelRemesh.invalidInput "Ungültige Eingabe!"
alert.matrix.clone "Das Objekt wird dupliziert"
alert.gizmo.usePivot "Benutzerdefinierten Drehpunkt (Pivot) verwenden."
alert.gizmo.useAuto "Automatischen Drehpunkt (Pivot) verwenden."
alert.gizmo.editPivot "Aktueller Modus: Drehpunkt (Pivot) bearbeiten"
alert.gizmo.editObject "Aktueller Modus: Objekt bearbeiten."
alert.dynamic.enable "Dynamic Topology ist aktiviert!"
alert.dynamic.disable "Dynamic Topology ist deaktiviert!"
alert.colorPicker "Ziehen Sie Ihren Finger auf das Mesh, um eine Farbe auszuwählen."
alert.backgroundTransform "Einfaches Antippen, um den Transformationsmodus zu verlassen."
alert.camera.resetView "Reset Ansicht"
alert.camera.snapView "Snap-Ansicht"
alert.mask.show "Maske anzeigen"
alert.mask.hide "Maske ausblenden" 
alert.selection.lock "Auswahl sperren"
alert.selection.unlock "Auswahl entsperren"
alert.selection.isolate "Auswahl isolieren"
alert.selection.showAll "Alles anzeigen"
alert.quickSave "Speichere..."
alert.forceShowPainting.fill "Painting anzeigen aktiviert, [Paint all] wurde verwendet."
alert.forceShowPainting.tool "Painting anzeigen aktiviert, Objekt painted."
alert.multiresLost "Multiresolution geht verloren!"
alert.rangeWarning "Der Detailgrad ist hoch und kann viel Speicherplatz erfordern!"
// autosave popup
alert.autoSave.auto "Automatisches Speichern in... $0s"
// bottom warning
alert.warning.needLayer "Das aktuelle Werkzeug erfordert eine aktive Ebene."
alert.warning.multiresLost "Multiresolution geht verloren."
alert.warning.paintingHidden "Painting ausgeblendet: Im Einstellungsfenster wieder einblenden."
alert.warning.noPartialWireframe "Das partielle Zeichnen ist deaktiviert, wenn das Drahtgitter (Wireframe) eingeblendet ist."
// bottom tip
alert.tip.shapeOrthographic "Erwägen Sie die Verwendung einer orthografischen Kamera, wenn Sie perspektivische Verzerrungen bei der Verwendung von Screen Project vermeiden möchten."
// undo
alert.state.trial "Rückgängig abgebrochen: Sie benutzen die Testversion!"

// --------------------------------------------------------------------------------------
// background
background "Hintergrund"
background.settings "Einstellungen" // unused
background.color "Farbe" // unused
background.environment "Umgebung"
background.blur "Unschärfe"
background.exposure "Belichtung"

background.imageEnable "Referenzbild"
background.imageOverlay "Overlay"
background.imageAlpha "Alpha"
background.imageReset "Reset Einstellungen"
background.imageTransform "Umwandeln"
// transform
background.imageX "X-Position"
background.imageY "Y-Position"
background.imageRotation "Rotation"
background.imageScale "Skalierung"

// --------------------------------------------------------------------------------------
// camera
camera "Kamera"
// saved views
camera.updateView "Ansicht updaten?"
camera.addView "Ansicht hinzufügen"
camera.focusOn "Fokus auf"
// projection
camera.projection "Projektion"
camera.orthographic "Orthographisch"
camera.perspective "Perspektivisch"
camera.fov "Vertikales FoV"
camera.focal "Brennweite $0mm (35mm Sensor)"
// orbit
camera.orbit "Orbit mode"
camera.orbit.help "Trackball gives more degree of freedom you can also roll the camera with 2 fingers."
camera.trackball "Trackball"
camera.turntable "Turntable"
// speed
camera.speed "Geschwindigkeit"
camera.rotation "Rotation"
camera.panning "Schwenken (Pan)"
camera.zooming "Zoomen"
// misc
camera.resetView "Reset Ansicht"
camera.snapView "Snap Ansicht"
// interaction
camera.pivot "Pivot"
camera.doubleTapMesh "Doppel-Tap Mesh"
camera.doubleTapBackground "Doppel-Tap Hintergrund"
camera.doubleTapPivot "Update bei Doppel-Tap"
camera.doubleTapPivot.help "Aktualisieren Sie den Drehpunkt (Pivot) beim doppelten Antippen."
camera.autoPivot "Bei Kamerabenutzung"
camera.autoPivot.help "Aktualisieren Sie den Drehpunkt (Pivot), wenn Sie beginnen, mit der Kamera zu hantieren."
camera.doubleTapFocus "Fokus"
camera.doubleTapFocus.help "Wenn Sie doppelt auf das Mesh tippen, schwenkt die Kamera und fokussiert auf den ausgewählten Punkt."
camera.doubleTapFocusSelection "Fokus auf Auswahl"
camera.doubleTapFocusSelection.help "Wenn Sie doppelt auf den Hintergrund tippen, wird der Fokus auf das ausgewählte Mesh anstatt auf die gesamte Szene gelegt."

// scene and layer lists
curve.preset "Preset"
curve.custom "Benutzerdefiniert"

// --------------------------------------------------------------------------------------
// debug
debug.uvPrimitive.warning "Deaktivieren Sie diese Option, wenn Sie keine UVs benötigen (zusätzlicher Speicher)."
debug.uvPrimitive "Primitive UVs beibehalten"
debug.uvPrimitive.help "Momentan werden nur Box und Kugel (Sphere) unterstützt.

Andere Typen werden in Zukunft unterstützt werden."
debug.uvNormalize "UVs normalisieren"
debug.uvNormalize.help "Nomad normalisiert die UVs innerhalb Tile [0-1]."
debug.uvBFF "BFF UVs hinzufügen"
debug.uvBFF.help "Hinzufügen einer alternativen Unwrapping-Methode (boundary first flattening - BFF).

Beachten Sie, dass BFF zu Überschneidungen führt, wenn Ihre Mesh-Topologie nicht aus einer Scheibe oder Kugel besteht."
debug.logs "Logs"
debug.heightmap "Heightmap"
debug.graphics "Grafik"
debug.thumbnails "Vorschaubilder erstellen"

// scene and layer lists
expandList "UI: Liste erweitern"
expandList.help "Nur eine UI-Option zur einfacheren Listenverwaltung."

// --------------------------------------------------------------------------------------
// file
file.project.empty "Sie haben noch kein Projekt gespeichert!"
file.project.unsaved "Nicht gespeicherte Änderungen!"
file.project.loseUnsaved "Sie werden nicht gespeicherte Änderungen verlieren!"
file.project.lastManualSave "Vorschau der letzten manuellen Speicherung"
file.project.trialNoOpen "Testversion: Sie können das aktuelle Projekt nicht mehr öffnen!"
file.project.trialOnlyOpen "Testversion: Sie können nur Ihr aktuelles Projekt wieder öffnen!"

// --------------------------------------------------------------------------------------
// project
file.project "Projekt"
file.project.save "Speichern"
file.project.save.confirm "$0 speichern?"
file.project.saveAs "Speichern als"
file.project.saveAs.confirm "$0 überschreiben?"
file.project.open "Öffnen"
file.project.open.confirm "$0 öffnen?"
file.project.add "Hinzufügen"
file.project.add.confirm "$0 der Szene hinzufügen?"
file.project.new "Neu"
file.project.new.confirm "Neue Szene erstellen?"
file.project.delete "Löschen"
file.project.delete.confirm "$0 löschen?"
file.project.delete.confirmActive "$0 löschen?

Dies ist das derzeit aktive Projekt!"
file.project.delete.confirmOk "Sind Sie sicher?"
file.project.rename "Umbenennen"

// autosave
file.project.autoSave "Auto-Speichern"
file.project.autoSave.confirm "Automatisches Speichern deaktivieren?"
file.project.autoSave.help "Speichert Ihr Projekt in regelmäßigen Abständen in einer separaten Datei.
Die automatisch gespeicherte Datei finden Sie in:

$0"
file.project.autoSave.popup "Popup-Zeitüberschreitung"
file.project.autoSave.minutes "Timer Popup"
file.project.autoSave.delete "AutoSave löschen"
file.project.autoSave.delete.confirm "Bestätigen?"

// import
file.import.title "Import"
file.import.title.help "Unterstützte Formate:
- Wavefront (.obj)
- glTF 2.0 (.glb .gltf)
- STL (.stl)"
file.importOpen "Öffnen"
file.importOpen.confirm "Neue Datei importieren?"
file.import.add "Der Szene hinzufügen"
file.import.add.confirm "Neue Datei importieren?"

file.exportSelection "Nur Auswahl exportieren"
file.exportSelection.help "Exportieren Sie nur das aktuell ausgewählte Mesh anstelle der gesamten Szene."
file.convertToQuad "Vierecke wiederherstellen"
file.convertToQuad.help "Wiederherstellen von Vierecken aus Dreiecken durch Paarung von Dreiecken (wenn sie in den Dateien benachbart sind)."

// export
file.export.title "Export"
file.export.title.help "Wählen Sie den glTF-Export falls möglich. Das glTF-Format unterstützt beim Export mehr Funktionen als andere Formate.

Nicht jedes Programm kann allerdings glTF importieren."

// generic export
file.export.texture "Texturen exportieren"
file.export.texture.help "Mit dieser Option werden keine Vertex-Farben in die Texturen eingefügt (kein Baking).

Es werden nur dann Texturen neu exportiert, wenn sie bereits in einer importierten Datei vorhanden waren."
file.export.normal "Normale exportieren"
file.export.normal.help "Aktivieren Sie diese Option, wenn Sie die Datei in einer anderen Software öffnen möchten.

Nomad ignoriert immer die Normale, da es sie neu berechnet."

// gltf
file.export.gltf "glTF 2.0 exportieren"
file.export.gltfLayer "Ebenen exportieren"
file.export.gltfLayer.help "Exportieren Sie Ebenen als Morphs. Offiziell von glTF unterstützt, so dass es auch auf anderen Softwares funktionieren sollte."
file.export.gltfColor "Vertex-Farben exportieren"
file.export.gltfColor.help "Exportieren Sie Vertex-Farben. Offiziell von glTF unterstützt, daher sollte es auch mit anderen Programmen funktionieren."
file.export.gltfExtraPaint "Zusätzliche Material-Ebenen exportieren"
file.export.gltfExtraPaint.help "Exportieren Sie Roughness, Metalness, Masken und Ebenen-Painting. Dies wird von anderen Programmen als Nomad ignoriert werden."

// obj
file.export.obj "OBJ exportieren"
file.export.objWarning "Ebenen und zusätzliches Painting (Roughness, Metalness, Masken) gehen verloren."
file.export.objColorAppend "Vertex-Farben exportieren"
file.export.objColorAppend.help "Farbinformationen nach Vertices einfügen.

Einige 3D-Programme können dies importieren, aber nicht alle."
file.export.objColorHexa "Hexa-Farbe"
file.export.objColorHexa.help "Farbe als Hexadezimalwert schreiben (zBrush-Methode).

Einige 3D-Programme können dies importieren, aber nicht alle."

// stl
file.export.stl "STL exportieren"
file.export.stlWarning "Ebenen und zusätzliche Painting (Roughness, Metalness, Masken) gehen verloren."
file.export.stlColor "Vertex-Farben exportieren"
file.export.stlColor.help "Einige 3D-Programme können dies importieren, aber nicht alle."
file.export.stlAscii "Standardmäßig ist das Format binär.

Sie können auch in das Textformat (ASCII) exportieren, allerdings wird die Datei dann größer."

file.settings.title "Einstellungen"
file.settings.title.help "Hier werden die meisten Einstellungen der App gespeichert (Kamera, Interface usw.).

Einige Ressourcen werden separat und automatisch gespeichert, dazu gehören:

- HDRIs
- MatCaps
- Alphas
- Texturen
- Hintergründe
- Projekte

Im Moment können die Pinsel-Einstellungen nicht gespeichert werden, aber eine benutzerdefinierte Pinsel-Verwaltung ist geplant."

// settings
file.settings.reset "Reset"
file.settings.reset.confirm "Alle Einstellungen zurücksetzen?

Projekte, Alphas, MatCaps, HDRIs und Hintergründe sind davon nicht betroffen."

// materials
file.materials "Materialbibliothek"
file.materials.reset "Reset"
file.materials.reset.confirm "Materialbibliothek zurücksetzen?"

// tools
file.tools "Werkzeug-Voreinstellungen"
file.tools.reset "Reset"
file.tools.reset.confirm "Werkzeug-Voreinstellungen zurücksetzen?"

// render
file.render "Render"
file.render.showInterface "Interface anzeigen"
file.render.renderRatio "Renderfaktor"
file.render.renderRatio.help "Ein Wert von 1,0 bedeutet, dass Nomad mit der gleichen Auflösung rendert, wie die unten angegebene Bildgröße.

Verwenden Sie diese Option, wenn Sie bei einer bestimmten Auflösung nicht rendern können (Abstürze wegen Speichermangels)."
file.render.help "Renderfaktor"
file.render.size "Endgültige Größe"
file.render.size.custom "Benutzerdefiniert"
file.render.screenResolution "Bildschirm"
file.render.export "PNG exportieren"
file.render.width "Breite"
file.render.height "Höhe"
file.render.warn "Die Export-Auflösung ist hoch ($0x$1)!

Stellen Sie sicher, dass Sie Ihr Projekt vorher speichern, für den Fall, dass Ihr Gerät keinen VRAM mehr hat und abstürzt."
file.render.transparent "Transparenter Hintergrund"
file.render.transparent.help "Diese Option kann nützlich sein, wenn Sie das freigestellte Mesh in eine 2D-Software importieren möchten.

Teilweise Objekttransparenz wird momentan noch nicht unterstützt."

// --------------------------------------------------------------------------------------
// gesture menu
gesture.useGlobal "Globale Vorgabe nutzen"
gesture.useGlobal.help "Standardmäßig haben alle Werkzeuge dieselben Pressure-Settings.

Deaktivieren Sie diese Option, wenn Sie spezielle Pressure-Settings für dieses Werkzeug wünschen."

gesture.pressure "Pressure"
gesture.pressureTitle "Pressure ($0)"
gesture.pressure.noTool "Dieses Werkzeug verwendet keine Pressure-Settings für den Stift."
gesture.pressure.noGrab "Grab ignoriert Pressure-Settings."
gesture.pressure.radius "Radius"
gesture.pressure.intensity "Stärke" 
gesture.pressure.useRadius "Aktiv"
gesture.pressure.useIntensity "Aktiv" 
gesture.pressure.curveRadius "Radius"
gesture.pressure.curveIntensity "Stärke"

gesture.cameraInteraction "Kamera:"
gesture.sculptInteraction "Sculpt:"
gesture.interaction.fingerAndStylus "Finger und Stylus"
gesture.interaction.finger "Finger"
gesture.interaction.stylus "Stylus"

gesture.fingerLighting "Licht-Rotation (3 Finger)"
gesture.fingerLighting.help "Bewegen Sie 3 Finger horizontal (von links nach rechts oder umgekehrt) auf der dem Arbeitsbereich, um die Umgebung, die Lichter und das MatCap zu rotieren."
gesture.fingerRadius "Tool-Radius bearbeiten (3 Finger)"
gesture.fingerRadius.help "Bewegen Sie 3 Finger horizontal auf der dem Arbeitsbereich (von oben nach unten = verkleinern, von unten nach oben vergößern), um den Werkzeug-Radius zu verändern."

gesture.fingerSmooth "Finger glättet immer"
gesture.unknownPressure "Unerkannten Pressure zulassen"
gesture.unknownPressure.help "Aktivieren Sie diese Option, wenn Pressure (Druck) mit Ihrem Stift nicht funktioniert oder wenn Sie einen Pressure für den Finger benötigen." 

// pencil
gesture.pencilAction.none "Nichts"
gesture.pencilAction.smooth "Smooth"
gesture.pencilAction.alt "Add/Sub"
gesture.pencilAction.android "Stift: Taste"
gesture.pencilAction.android.help "Experimentell"
gesture.pencilAction.ios "Stift: Doppel-Tap"
gesture.pencilAction.ios.help "Nur aktiv für Apple Pencil 2. Generation."

// history
gesture.history "Schnelle-Geste"
gesture.history.help "2-Finger Tap für UnDo.

3-Finger Tap für ReDo."

// size rejection
gesture.useSizeRejection "Touch Schwelle"
gesture.useSizeRejectionConfirm "Wenn Sie Probleme bei der Erkennung der Touch-Eingabe haben, stellen Sie sicher, dass diese Option deaktiviert ist!"
gesture.useSizeRejection.help "Die Eingabe auf dem Touchdisplay wird abgelehnt, wenn die Größe der Kontaktfläche beim Touch größer ist, als der vorgegebene Wert.

Funktioniert möglicherweise nicht auf jedem Gerät."
gesture.sizeRejection "Schwelle für maximale Größe"
// help
gesture.interaction.title "Gesten" 
gesture.interaction.title.help "Diese Optionen gelten immer global."

// --------------------------------------------------------------------------------------
// history
history "Verlauf"
history.root "Root"
history.undoConfirm "Bestätigen Sie das UnDo (Rückgängig machen) all dieser Änderungen?"
history.undoWarning "Wenn Sie eine nachträgliche Bearbeitung vornehmen, können viele Änderungen verloren gehen."
history.stack "Stapel"
history.limitSize "Verlaufslimit (MB)"
history.limitSize.help "Max. Größe des Verlaufs (in MB).

Der Verlauf wird bei der nächsten aufgezeichneten Änderung aktualisiert."
history.limitStack "Stapel-Limit"
history.limitStack.help "Maximale Anzahl von Änderungen, die im Verlauf bleiben.

Der Verlauf wird bei der nächsten aufgezeichneten Änderung aktualisiert."
history.rangeProtect "Schutz des Bereichs"
history.rangeProtect.help "Wenn Sie weit in der Verlaufsliste zurückgehen, wird ein Bestätigungsdialog angezeigt, bevor viele Änderungen auf einmal rückgängig gemacht werden."
history.restoreCamera "Reset Kamera"
history.restoreCamera.help "Aktivieren Sie diese Option, um die gespeicherte Kamera-Ansicht wiederherzustellen, wenn Sie eine Aktion rückgängig machen/wiederherstellen (UnDo/ReDo)."
// display undo/redo
history.state.undo "UnDo: $0"
history.state.redo "ReDo: $0"
history.state.symmetrySplit "Symmetrie Split"
history.state.voxelRemesh "Voxel Remesh"
history.state.surfaceRemesh "Surface Remesh"
// state multires
history.state.multiresToDynamic "Multires zu Dynamisch"
history.state.multiresLevel "Auflösung ändern"
history.state.multiresSubdivide "Subdivide"
history.state.multiresReverse "Reversion"
history.state.multiresDeleteLower "Low-Res löschen"
history.state.multiresDeleteHigher "High-Res löschen"
// mesh
history.state.meshDynamicToStatic "Dynamisch zu Statisch"
history.state.meshStaticToDynamic "Statisch zu Dynamisch"
history.state.meshSymmetryUpdate "Symmetrie-Update"
history.state.meshMatrixUpdate "Matrix-Update"
history.state.meshVisibility "Sichtbarkeit"
history.state.meshMaterial "Material ändern"
// state scene
history.state.sceneAddRemove "Szene"
history.state.sceneMeshOrder "Mesh Reihenfolge"
// state layer
history.state.layerOrder "Layer $0 Reihenfolge"
history.state.layerMergeRedo "Layer $0 getrennt"
history.state.layerCreate "Layer $0 erstellt"
history.state.layerDelete "Layer $0 gelöscht"
history.state.layerMerge "Layer $0 verschmolzen"
history.state.layerHide "Layer $0 ausgeblendet"
history.state.layerShow "Layer $0 eingeblendet"
history.state.layerSelect "Layer $0 ausgewählt"
history.state.layerUnselect "Layer $0 deselektiert"
history.state.layerName "Layer $0 umbenannt"
history.state.layerFactor "Layer $0 Wert"
history.state.layerFactorOffset "Layer $0 Offset-Wert"
history.state.layerFactorColor "Layer $0 Color-Wert"
history.state.layerFactorRoughness "Layer $0 Roughness-Wert"
history.state.layerFactorMetalness "Layer $0 Metalness-Wert"
// state light
history.state.lightVisible "Licht $0 sichtbar"
history.state.lightIntensity "Licht $0 Stärke"
history.state.lightColor "Licht $0 Farbe"
history.state.lightPosition "Licht $0 Position"
history.state.lightShadow "Licht $0 Schatten"
history.state.lightBias "Licht $0 Schatten-Bias"
history.state.lightAttachment "Licht $0 verbunden"
history.state.lightAdd "Licht $0 hinzugefügt"
history.state.lightDelete "Licht $0 gelöscht"
history.state.lightCopy "Licht $0 kopiert"
history.state.lightMove "Licht $0 bewegt"
history.state.lightType "Licht $0 Typ"
history.state.lightSpotAngle "Licht $0 Spot-Winkel"
history.state.lightSpotSoftness "Licht $0 Spot-Santfheit"
// state view
history.state.viewAdd "Ansicht $0 hinzugefügt"
history.state.viewMove "Ansicht $0 Reihenfolge"
history.state.viewDelete "Ansicht $0 gelöscht"

// --------------------------------------------------------------------------------------
// interface
interface "Interface"

// bottom buttons
interface.bottomButtons "Shortcuts hinzufügen (unten)..."
interface.shortcut.voxelRemesh "Voxel Remesh"
interface.shortcut.wireframe "Wireframe"
interface.shortcut.lockSelection "Selektion einfrieren"
interface.shortcut.lockSelection.help "Wenn diese Funktion aktiviert ist, können Sie die Auswahl nicht ändern, indem Sie auf ein Mesh tippen."
interface.shortcut.cameraReset "Kamera-Reset"
interface.shortcut.cameraSnap "Kamera-Snap"
interface.shortcut.perspective "Perspektive"
interface.shortcut.cameraSnapFlip "Flip bei Snap"
interface.shortcut.cameraSnapFlip.help "Wenn die Kamera bereits eingerastet ist, kann mit diesem Shortcut die Ansicht gespiegelt werden."

// colors
interface.colors "Hauptfarben"
interface.colorSelect "Widget-Farbe"
interface.colorBase "Grundfarbe"
interface.colorBaseTransparent "Panel-Farbe" 
interface.panelTransparent "Panel transparent"
interface.blurFactor "Unschärfe"

// color preset
interface.colorsPresets "Farb-Presets"
interface.presetBlurRed "Rot"
interface.presetBlurBlue "Blau"
interface.presetBlurGreen "Grün"
interface.presetBlurYellow "Gelb"
interface.presetBlackWhite "S/W"
interface.presetWhiteBlack "W/S"
interface.presetLividOrange "Livid & Orange"
interface.presetCardboard "Cardboard"
interface.presetDefault "Standard"

// style
interface.style "Stil"
interface.resetAll "Reset Interface"
interface.resetAll.confirm "Alle Interface-Einstellungen zurücksetzen?"
interface.flipTop "Oben spiegeln"
interface.flipBottom "Unten spiegeln"
interface.flipMiddle "Mitte spiegeln"
interface.showTooltips "Tooltips anzeigen"
interface.showTooltips.help "Dies ist ein Tooltip."
interface.materialPreview "Material-Picker Vorschau"
interface.toolboxHide "Toolbox autom. ausblenden"
interface.toolboxHide.help "Aktivieren Sie diese Option, wenn Sie die Toolbox ausblenden möchten."
interface.toolboxMaxColumn "Toolbox max. Spalten"
interface.toolboxResetOrder "Reset Anordn. Tools"
interface.rounding "Ecken abrunden"
interface.curveToolSymmetric "Symmetric tool curve widget"
interface.curveToolSymmetric.help "The widget can be found in the Tool panel under the falloff option."
interface.scale "Gesamt-Skalierung"
interface.cursorStep "Vertikale Abstände"
interface.panelWidth "Panel-Breite"
interface.fontScale "Schriftgröße"

// --------------------------------------------------------------------------------------
// layer sub menu
layer.action "Aktion"
layer.name "Name"
layer.delete "Löschen"
layer.move "Bewegen"
layer.duplicate "Duplizieren"
layer.mergeDown "Merge down"
layer.factors "Kanal-Werte"
layer.offsetFactor "Position"
layer.colorFactor "Farbe"

// --------------------------------------------------------------------------------------
// layers menu
layers.addLayer "Layer hinzufügen"
layers.addLayerTrial "Die Testversion ist auf 1 Layer je Mesh beschränkt."
layers.title "Layer"
layers.title.help "Ebenen (Layer) können Positionsverschiebungen und Painting aufzeichnen, was für einen nicht-linearen Arbeitsablauf nützlich sein kann.
Zum Beispiel durch das Experimentieren mit verschiedenen Gesichtsausdrücken, ohne sich auf den Verlauf zurückzugreifen, um die Änderungen rückgängig zu machen.

Beim Painting werden die Ebenen von oben nach unten sortiert, d. h. die obersten Ebenen verdecken die unteren.

Um die Ebenendeckkraft (Layer Opacity) aufzulösen, teilen sich alle Painting-Daten (Color, Roughness, Metalness) die gleiche Maske.
Sie können einen Teil dieser Maske (und damit den Einfluss dieser Ebene) zurücksetzen, indem Sie das Werkzeug 'DelLayer' verwenden."
layers.primitive "Ebenen sind für Grundobjekte (Primitives) nicht verfügbar."
layers.baseSelected "Nichts"

// --------------------------------------------------------------------------------------
// light sub menu
light "Licht"
light.color "Farbe"
light.intensity "Stärke"
light.attachment "Anordnung"
light.attachment.fixed "Fixiert"
light.attachment.camera "Kamera"
light.attachment.environment "Environment"
light.attachment.help "-- Fixiert
Die Ausrichtung des Lichts wird sich nicht ändern.

-- Kamera
Die Ausrichtung des Lichts hängt von der Kamera-Ansicht ab."
light.type "Typ"
light.type.directional "Direktional"
light.type.spot "Spot"
light.type.point "Punkt"
light.spotAngle "Winkel Lichtkegel"
light.spotSoftness "Sanftheit"
light.shadowCast "Schatten"
light.shadowNormalBias "Normal-Bias"
light.visible "Zeigen"
light.resetPosition "Zentrieren"

// --------------------------------------------------------------------------------------
// material
material "Material"
material.addNew "Hinzufügen"
// if the shading mode is in matcap or unlit
material.unlitWarning "Roughness und Metalness werden im aktuellen Shading-Modus irgnoriert."
// refraction
material.ior "Lichtbrechungsindex (Refraction)"
material.paintingOverride "Painting aufheben"
material.refractionSurfaceGlossiness "Oberflächenglanz"
material.refractionSurfaceGlossiness.help "- bei 0 nutzt die Oberfläche painted Roughness
- bei 1 ist die Oberfläche völlig glatt"
material.refractionInteriorRoughness "Rauheit innen"
material.refractionInteriorRoughness.help "- bei 0 wird innen painted Roughness benutzt
- bei 1 ist das Innere völlig rau"
material.paintGlossy "Paint Glossy"
material.paintGlossy.help "Painting mit Wert Roughness und Metalness von jeweils 0, für eine eine scharfe Brechung.

Dies entspricht der Nutzung des Paint All-Features aus dem Paint-Menü  mit deaktivierten Color- und Roghness-Kanälen"
// absorption
material.absorptionEnable "Absorption"
material.absorptionEnable.help "Simulieren Sie die Absorption des Lichts, wenn es das Volumen durchbricht.

Dünne Teile werden hell, da sie mehr Licht durchlassen, während dicke Bereiche dunkler sind..

Der Effekt hängt stark von der Mesh-Geometrie ab, es wird nur eine Annäherung an die Mesh-Dicke verwendet."
material.absorptionFactor "Faktor"
// alpha
material.opacity "Opacity"
material.alphaMode.opaque "Opaque"
material.alphaMode.blending "Blending"
material.alphaMode.additive "Additive"
material.alphaMode.dithering "Dithering"
material.alphaMode.dithering.help "Das Dithering (Fehlerdiffusion) ist eine Technik um bei Bildern die Illusion einer größeren Farbtiefe zu erzeugen. Dithering ist eine Art des Rasterns."
material.alphaMode.refraction "Refraction"
// shadows
material.castShadows "Wirft Schatten"
material.receiveShadows "Empfängt Schatten"
// backface
material.twoSided "Beidseitig"
material.alwaysUnlit "Immer Unlit"
material.flipCulling "Umgekehrtes Culling"
// material
material.reflectance "Reflexionsgrad"
material.reflectance.help "Kontrollieren Sie den Grad der Reflexion, den das Material bei nicht-metallischen Materialien erhält.

In den meisten Fällen sollte der Standardwert verwendet werden (0,5 - was dem Standardwert von 4% reflektiertem Licht bei normalem Winkel entspricht)."

// --------------------------------------------------------------------------------------
// menu name (visible on small screen menu are collapsed)
menu.files "Dateien"
menu.scene "Szene"
menu.multires "Multires"
menu.voxel "Voxel"
menu.dyntopo "Dyntopo"
menu.topology "Deci/UV..."
menu.primitive "Grundformen"
menu.render "Render"
menu.material "Material"
menu.postProcess "Nachbearbeitung"
menu.camera "Kamera"
menu.background "Hintergrund"
menu.tool "Tool"
menu.stroke "Stroke"
menu.paint "Paint"
menu.symmetry "Symmetrie"
menu.pressure "Pressure"
menu.layers "Layer"
menu.settings "Einstellungen"
menu.interface "Interface"
menu.history "Verlauf"
menu.historySettings "Einstellungen"
menu.about "Über"
menu.debug "Debug"

// --------------------------------------------------------------------------------------
// mesh sub menu
mesh.action "Aktion"
mesh.holeClose "Löcher schließen"
mesh.holeDetail "Detail"
mesh.separate "Trennen"
mesh.triplanarWarning "Layer, Painting und Multiresolution geht verloren!"
mesh.triplanarResolution "Auflösung"
mesh.triplanarCubic "Würfelform erzwingen"
mesh.triplanarConvert "Konvertieren"
mesh.name "Name"
mesh.type "Typ"
mesh.typeStatic "Statisch"
mesh.typeMultiresolution "Multiresolution"
mesh.typeDynamic "Dynamisch"

// --------------------------------------------------------------------------------------
// painting
paint.useGlobal "Globales Material"
paint.useGlobal.help "Wenn diese Option aktiviert ist, ist das ausgewählte Material dasselbe wie bei den anderen Werkzeugen.

Beachten Sie, dass hier nur die Einstellungen für Roughness, Metalness und Color berücksichtigt werden."
paint.usePainting "Stroke painting" 
paint.intensity "Paint Stäke"
paint.paintAll "Füllen" 
paint.paintAll.help "Wendet das aktuelle Material auf das gesamte Mesh ohne maskierte Bereiche an (Paint all).

Maskierte Bereiche und deaktivierte Kanäle werden berücksichtigt und entsprechend ausgespart!"
paint.paintAllForce "Füllen erzwingen"
paint.paintAllForce.help "Wendet das aktuelle Material auf das gesamte Mesh inklusive maskierter Bereiche an (Force paint all).

Maskierte Bereiche und deaktivierte Kanäle werden NICHT berücksichtigt. Es wird wirklich das gesamte Mesch mit dem Material überzogen!"
paint.strokePaintingTitle "Painting ($0)"
paint.layerWarning "Die Kanalmaskierung wird ignoriert, wenn Sie versuchen, sie auf eine Ebene anzuwenden."
paint.texture.title "Textur"
paint.texture.title.help "Ein Bild, das den Brush Stroke färbt.

Beachten Sie, dass Tiling und Skalierung des Alphas genutzt werden."
paint.texture.warningEnable "Stroke Painting muss aktiviert sein, um Texturprojektion zu ermöglichen (Kontrollkästchen oben)!"
paint.texture.warningIgnored "Das aktuelle Werkzeug kann keine Texturen verwenden!"
paint.useAlpha "Stroke Alpha benutzen"
paint.useAlpha.help "Verwenden Sie das Alpha-Set im Stroke-Menu um das Painting zu beeinflussen."
paint.useFalloff "Use stroke falloff"
paint.useFalloff.help "Nutzen Sie das Falloff im Stroke-Menu um das Painting zu beeinflussen."

// --------------------------------------------------------------------------------------
// popup (for example tap on a tool, to open edit popup)
popup.save "Speichern"
popup.save.confirm "Speichern bestätigen?"
popup.lastSave "Letzte Speicherung"
popup.lastSave.confirm "Letzte Speicherung laden?"
popup.reset "Reset"
popup.reset.confirm "Zurücksetzen bestätigen?"
popup.clone "Klonen"
popup.rename "Umbenennen"
popup.delete "Löschen"
popup.delete.confirm "Löschen bestätigen?"
popup.delete.confirm.yes "JA, löschen"

// title when requesting input value through virtual keyboard
input.name "Name"
input.number "Wert"

// --------------------------------------------------------------------------------------
// postprocess
postprocess.mainEnable "Nachbearbeitung (Post)" 
postprocess.quality "Qualität"
postprocess.quality.help "Aktivieren Sie diese Optionen, um die Qualität auf Kosten der Leistung zu verbessern.

It will improve:
- Reflektionen
- Ambient Occlusion (AO)
- Tiefenschärfe (DoF - Depth Of Field)
"
postprocess.maxSamples "Max. Samples"
postprocess.fullResolution "Volle Auflösung"
// fxaa
postprocess.fxaaEnable "Anti-Aliasing (FXAA)"
// taa
postprocess.taaEnable "Anti-Aliasing (TAA)"
postprocess.taaEnable.help "Verringert das Flackern, wenn Sie die Kamera bewegen."
// ssr
postprocess.ssrEnable "Reflektion (SSR)" 
postprocess.ssrFactor "Stärke" 
postprocess.ssrDistanceFading "Distance-Fading" 
postprocess.ssrDistanceFading.help "Dämpfen Sie die Wirkung je nach Entfernung der Reflexion.
Es kann helfen, Artefakte zu verbergen, unter denen die SSR leidet."
postprocess.ssrUnlitWarning "SSR ist nur im PBR-Shading-Modus wirksam."
// ssao
postprocess.ssaoEnable "Ambient Occlusion (AO)" 
postprocess.ssaoRadius "Größe" 
postprocess.ssaoFactor "Stärke" 
postprocess.ssaoBias "Wölbungs-Bias" 
postprocess.ssaoBias.help "Wie empfindlich der Effekt ist, hängt von der Oberflächenwölbung ab."
// dof
postprocess.dofEnable "Depth Of Field (DoF)"
postprocess.dofBlurFar "Ferne Unschärfe" 
postprocess.dofBlurNear "Nahe Unschärfe"
postprocess.dofFocusTip "Tippen Sie auf ein Objekt, um den Fokuspunkt zu ändern."
// bloom
postprocess.bloomEnable "Leuchten (Bloom)" 
postprocess.bloomIntensity "Stärke" 
postprocess.bloomRadius "Radius" 
postprocess.bloomRadius.help "Wie ausgedehnt das Leuchten ist."
postprocess.bloomThreshold "Schwellenwert" 
postprocess.bloomThreshold.help "Schwellenwert für die Leuchtkraft (Luminosity), um zu entscheiden, ob ein Pixel das Leuchten (Bloom) emittiert oder nicht.
Steht der Wert auf 0, leuchtet alles."
// tone mapping
postprocess.toneEnable "Tone-Mapping" 
postprocess.toneExposure "Belichtung" 
postprocess.toneContrast "Kontrast" 
postprocess.toneSaturation "Sättigung" 
postprocess.toneMappingNone "None"
// curve
postprocess.curveEnable "Color-Grading"
postprocess.curve.luminance "Luminanz"
postprocess.curve.red "Rot"
postprocess.curve.green "Grün"
postprocess.curve.blue "Blau"
postprocess.curveReset "Kanal neutral"
postprocess.curveResetAll "Alles neutral"
// chromatic
postprocess.chromaticEnable "Chromatische Aberration" 
postprocess.chromaticFactor "Stärke" 
// vignette
postprocess.vignetteEnable "Vignette" 
postprocess.vignetteSize "Größe" 
postprocess.vignetteHardness "Ausprägung" 
// sharpness
postprocess.sharpnessEnable "Schärfe" 
postprocess.sharpnessFactor "Stärke" 
// grain
postprocess.grainEnable "Körnung (Grain)" 
postprocess.grainFactor "Stärke" 
// curvature
postprocess.curvatureEnable "Curvature" 
postprocess.curvatureCavity "Cavity" 
postprocess.curvatureBump "Bump" 

// --------------------------------------------------------------------------------------
// primitive (scene menu)
primitive "Primitive"
primitive.box "Box"
primitive.sphereCube "Sphere"
primitive.sphereUV "UV Sphere"
primitive.icosahedron "Icosahedron"
primitive.cylinder "Cylinder"
primitive.cone "Cone"
primitive.torus "Torus"
primitive.lathe "Lathe"
primitive.tube "Tube"
primitive.plane "Plane"
primitive.triplanar "Triplanar"
primitive.needValidate "Primitives should be validated in order to be sculpted."

// for 3d editing in viewport
primitive.useFloatPanel "Panel inside viewport"
primitive.useFloatPanel.help "Move some of the primitives options directly in the viewport."
primitive.edit "Edit"
primitive.edit.help "Allow 3d editing in the viewport.

You can disable this feature if you want to interact with the Gizmo or the Transform tool modifying the primitive."

primitive.mainConfig "Parameter"
primitive.topology "Topology"
primitive.geometry "Geometry"

// common config
primitive.validate "Validieren"
primitive.maxFaces "Max Faces"
primitive.maxFaces.help "The maximum number of faces a primitive can have.

This limit is only active while the primitive is not validated, afterwards the safeguard is gone."
primitive.linear "Flat subdivision"
primitive.subdivision "Post subdivision"

// common config
primitive.radius "Radius"
primitive.size "Size"
primitive.sizeX "Size X"
primitive.sizeY "Size Y"
primitive.sizeZ "Size Z"
primitive.division "Division"
primitive.divisionX "Division X"
primitive.divisionY "Division Y"
primitive.divisionZ "Division Z"
primitive.angleX "Angle X"
primitive.angleY "Angle Y"
primitive.angleZ "Angle Z"
primitive.constantDensity "Constant density"
primitive.projectOnSphere "Project on sphere"
primitive.projectOnSphere.help "Snaps the points on a perfect sphere."

// triplanar
primitive.triplanar.title "Triplanar - Setting"
primitive.triplanar.title.help "Triplanar is using the mask information from 3 planes to fill a voxel grid that is then polygonized.

If you interact with the division or size sliders, the painting information will reset (smoothness is ok).

You should probably disable symmetry as it might not function as you would expect.

You can use the 'Topologically connected' option in the mask panel to paint a plane impacting the other planes."
primitive.triplanarSameSize "Same size (cube)"
primitive.triplanarPolish "Smoothness"
primitive.triplanarResetMask "Reset Maske"
primitive.triplanarReproject "Resize Maske"
primitive.triplanarReproject.title "Reproject the plane mask when updating the triplanar settings.
\
If you uncheck this option, it will revert to the default spherical masks."
primitive.isolate.all "All"
primitive.isolate.back "Back"
primitive.isolate.right "Right"
primitive.isolate.bottom "Bottom"
// plane
primitive.planeSameSize "Same size (square)"
// box
primitive.boxRegular "Same size (cube)"
// tube
primitive.tubeSnapOffset "Snap offset"
primitive.tubeSnapOffset.help "A value of 1.0 is equal to the tube's radius."
primitive.tubeThicknessStart "Radius start"
primitive.tubeThicknessEnd "Radius end"
// primitive.tubeTwist "Twist"
// primitive.tubeTwistRotate "Rotation"
// primitive.tubeTwistRadius "Magnitude"
// primitive.tubeTwistOffset "Offset"
primitive.tubeSnap "Snap"
// lathe
// torus
primitive.torusRadiusOuter "Radius outer"
primitive.torusRadiusInner "Radius inner"
primitive.torusAngle "Angle"
primitive.torusAngleOffset "Angle offset"
// cylinder
primitive.cylinderHeight "Height"
// cone
primitive.coneRadius "Radius"
primitive.coneHeight "Height"
// hole sub menu (cylinder, tube, etc)
primitive.hole "Hole"
primitive.hasHole "Has hole"
// both used for hole radius and main radius
primitive.radiusSync "Same radius"
primitive.radiusStart "Radius start"
primitive.radiusEnd "Radius end"
primitive.editRadius "Radius"
// spline (for lathe and tube)
primitive.spline "Spline"

// common resources stuffs
resource.delete "Delete"
resource.import "Import"

// --------------------------------------------------------------------------------------
// scene
scene.title "Scene"
scene.title.help "When using the selection checkbox, hold and drag your finger to select other objects easily."
scene.mergeSimple "Simple merge"
scene.mergeVoxel "Voxel merge"
scene.voxelResolution "Resolution"
scene.subtractionTip "Subtraction  : Hide mesh (eye icon)"
scene.intersectionTip "Intersection : All meshes hidden"

// --------------------------------------------------------------------------------------
// settings
settings.displayTitle "Display settings"
// wireframe
settings.wireframeTitle "Wireframe"
settings.wireframeDisplay "Wireframe"
settings.wireframeColor "Wireframe color"
settings.wireframeUV "UV 2d wireframe"
settings.wireframeUV.help "Display the wireframe UV in the background, if the model has UVs.

Note that when this option is enabled, it will also force the display of the checkerboard texture.

This option is used only if the shading mode is PBR - UV."
settings.debugUV "UV checkerboard"
settings.debugUV.help "Display a default texture for UV models that don't have any color texture.

This option is used only if the shading mode is PBR - UV."
// backface
settings.backfaceTitle "Two sided"
settings.backfaceVisible "Two sided"
settings.backfaceVisible.help "Faces will be visible from both sides."
settings.backfaceColor "Backface color"
settings.backfaceColored "Colored backface"
// outline
settings.outlineTitle "Outline"
settings.outlineEnable "Outline"
settings.outlineThickness "Thickness"
settings.outlineColor "Color"
// snap cube
settings.snapCubeTitle "Snap cube"
settings.snapCubeDisplay "Snap cube"
settings.snapCubeBottom "Bottom"
settings.snapCubeLeft "Left"
// stats
settings.statsTitle "Stats"
settings.statsDisplay "Stats"
settings.statsRight "Right"
settings.statsAll "Show full scene"
// grid
settings.gridTitle "Grid"
settings.gridDisplay "Grid"
settings.gridHeight "Height"
settings.gridColor "Color"
// cursor
settings.cursorWhileSculpting "Show circle while sculpting"
settings.cursorShowDot "Show small dot"
settings.cursorShowDot.help "The dot can appear as the camera pivot point or when you are sculpting."
settings.cursorShowRope "Show rope stabilizer"
// other
settings.renderRatio "Render resolution"
settings.darkenUnselected "Darken unselected meshes"
settings.smoothShading "Smooth shading"
settings.partialDraw "Partial drawing"
settings.partialDraw.help "Feature not polished!

Use it if you are sculpting a relatively small part of a high poly mesh.

It should make the sculpting smoother, but you should not enable wireframe!

Also it might add visual artefacts during the brush strokes"
settings.partialDrawWarning "Do not forget to turn off this option if the visual artefacts are too bothersome!"
settings.showPainting "Show painting"
settings.lightIcon "Light icons"
settings.lightIcon.help "Display light icons on the canvas so that you can select and edit them directly."
settings.holeTitle "Hole-filling"
settings.holeNonManifold "Fill non-manifold"
settings.holeNonManifold.help "Try to fill non manifold hole.
This option is not saved in the settings.
"
settings.loadGuiSettings "Keep gui settings (at import)"
settings.loadGuiSettings.help "When opening or importing a project file, all the gui-related settings embedded in the project will be loaded."
settings.loadObjKeepGroup "Keep OBJ groups"
settings.loadObjKeepGroup.help "When enabled, Nomad will split the OBJ each vertex group into separate objects."
settings.loadMergeLayers "Merge Layers (at import)"
settings.loadSkipTextures "Skip textures (at import)"
settings.loadKeepTopology "Keep topology (at import)"
settings.loadKeepTopology.help "Use this option if you don't want Nomad to fiddle with the topology of imported mesh.

It will disable vertex/face reordering, removal of vertex/face duplicates and removal of unused vertices."
// multires
settings.multiresTitle "Multiresolution"
settings.multiresMaxVertices "Max vertices count"
settings.multiresMaxVertices.help "Nomad doesn't perform memory check before subdivision, high poly count can easily lead to crashes."
settings.multiresLowResVertices "Low resolution threshold"
settings.multiresLowResVertices.help "A lower resolution of the mesh can be displayed when you move the camera.

You can increase this value if you want to display a higher resolution of the mesh."
// experimental
settings.experimentalTitle "Experimental"
settings.notSaved "These options are not saved in the settings."
// settings.parallel "Parallel sculpting"

// --------------------------------------------------------------------------------------
// shading
shading "Shading"
// main render mode
shading.pbr "PBR"
shading.matcap "Matcap"
shading.unlit "Unlit"
// lights
shading.lights "Lichter"
shading.lights.addLight "Licht hinzufügen"
shading.lights.unlitWarning "Lights are ignored in Matcap and Unlit mode."
// environment
shading.environment "Umgebung"
shading.environmentImport "HDR importieren"
shading.environmentExposure "Exposure"
shading.environmentRotation "Rotation"
shading.environmentRotation.help "You can rotate the environment by dragging 3 fingers horizontally on the viewport."
shading.environmentAttachedToCamera "Attached to camera"
shading.environmentAttachedToCamera.help "Attach the environment to the camera.

It will force the lighting to be consistent, which can be useful for sculpting purposes."
// matcap
shading.matcap "Matcap"
shading.matcapRotation "Rotation"
shading.matcapRotation.help "You can rotate the matcap by dragging 3 fingers horizontally on the viewport."
shading.matcapGlobal "Use global matcap"
shading.matcapGlobal.help "Uncheck this option to use a different matcap for this particular mesh."

// --------------------------------------------------------------------------------------
// bottom shortcut buttons (ICON FIT)
shortcut.voxel "Voxel"
shortcut.wire "Wire"
shortcut.mask "Maske"
shortcut.reset "Reset"
shortcut.snap "Snap"
shortcut.solo "Solo"
shortcut.lock "Lock"
shortcut.persp "Persp"

// --------------------------------------------------------------------------------------
// stat
stat.ramScene "Szene"
stat.vramScene "VRAM Szene"
stat.vramRender "VRAM Render"
stat.vramTextures "VRAM Texturen"
stat.ramHistory "Verlauf"
stat.ramOther "Anderes"
stat.usedMemory "RAM benutzt"
stat.freeMemory "RAM frei"
stat.ram "RAM"
stat.used "Benutzt: $0 MB"
stat.free "Frei: $0 MB"
stat.faces "Faces"
stat.triangles "Dreiecke"
stat.vertices "Vertices"
stat.quads "Quads"
stat.sceneFaces "Szenen Faces"
stat.sceneVertices "Szenen Vertices"

// --------------------------------------------------------------------------------------
// stroke
stroke "Stroke"
strokeTitle "Stroke ($0)"
stroke.useWorldRadius "World-space radius"
stroke.useWorldRadius.help "This is shared among every tools."
stroke.useShareRadius "Share radius"
stroke.useShareRadius.help "Share the radius value among every tools."
stroke.minSpacingAdjustIntensity "Adjust spacing intensity"
stroke.minSpacingAdjustIntensity.help "Adjust the brush intensity to ensure consistent deformation depending on the stroke spacing."
stroke.minSpacing "Stroke spacing"
stroke.minSpacing.help "Spacing between each stroke, relative to the tool radius.

Lower value will allow smoother stroke but performance will degrade."
stroke.lazySmooth "Stroke smoothing"
stroke.lazySmooth.help "Average multiple pointer position to get a smoother stroke.

With high values, the stroke will lag behind the pointer but will eventually catch up."
stroke.lazyRadius "Lazy rope stabilizer"
stroke.lazyRadius.help "Strokes will lag behind the pointer position according to a certain distance.

This can be used to draw smooth lines."
stroke.globalSettings "This is a global setting"
stroke.snapRadius "Snap radius"
stroke.snapRadius.help "Snap the stroke if the pointer lies close to the last recorded stroke.

This can be useful when drawing long continuous lines, while doing frequent pauses."
stroke.sculptOffset "Stroke offset"
stroke.sculptOffset.help "Apply a constant offset on the stroke.

This option is there to help for small screen when using fingers, so that your finger doesn't cover the stroke."
stroke.accumulate "Accumulate stroke"
stroke.accumulate.help "If this option is enabled, there is no limit to how much matter you can add/remove per stroke."
stroke.useDynamicTopology "Allow dynamic topology"
stroke.connectedTopology "Connected topology"
stroke.connectedTopology.help "This option will only sculpt the vertices that are linked to the picked surface.

This is typically used for the Move tool, for example if you want to exclusively move a part that self-intersect with another part."
stroke.onlyFrontFace "Front-facing vertex only"
stroke.onlyFrontFace.help "This option will ignore back facing vertices.

It can be useful if you want to paint part of a thin geometry without impacting the other side.

It also works for sculpting but you might experience some artefacts."
stroke.onlySameSide "Same-side vertex only"
stroke.onlySameSide.help "Ignore vertices that points in the opposite direction of the deformation."
stroke.curveFalloff "Falloff"
stroke.onlyLasso "Settings only active for the lasso tool."
// alpha
stroke.alpha "Alpha" 
stroke.alphaInvert "Invert value"
stroke.alphaWrap "Tiling"
stroke.alphaWrap.none "None"
stroke.alphaWrap.repeat "Repeat"
stroke.alphaWrap.mirror "Mirror"
stroke.alphaProject "Method"
stroke.alphaProject.surfaceContinuous "Surface"
stroke.alphaProject.screenFixed "Screen project"
stroke.alphaTiling "Tiling"
stroke.alphaScale "Scaling"
stroke.alphaScale.help "At minimum value, the alpha square is inside the tool circle radius."
stroke.alphaMidValue "Mid value"
stroke.alphaMidValue.help "Middle-point value at which no deformation occurs.

(Mid value = 0)
- Black: no displacement
- White: positive displacement

(Mid value = 0.5)
- Black: negative displacement
- White: positive displacement

(Mid value = 1)
- Black: negative displacement
- White: no displacement"
// stroke type
stroke.strokeType "Stroke type"
stroke.strokeTypeDot "Dot"
stroke.strokeTypeDrag "Drag"
stroke.strokeTypeGrab "Grab"
stroke.strokeTypeGrabRadius "Grab - dynamic radius"
stroke.strokeTypeGrabIntensity "Grab - dynamic intensity"

// --------------------------------------------------------------------------------------
// symmetry
symmetry "Symmetry"
symmetry.enable "Enabled"
symmetry.plane.title "Planes"
symmetry.toolIgnore "The current tool ignores symmetry."
symmetry.radial.title "Radial"
symmetry.radialX "Radial X"
symmetry.radialY "Radial Y"
symmetry.radialZ "Radial Z"
// method
symmetry.method "Method:"
symmetry.method.help "-- Local
The symmetry plane will move along the mesh when you use one of the transform tools (rotate, translate or gizmo).


-- World
The symmetry plane is fixed and will not move."
symmetry.methodWorld "World"
symmetry.methodLocal "Local"
// flip
symmetry.flip "Flip object"
// mirror
symmetry.mirror "Mirroring"
symmetry.mirror.help "Try to re-apply the symmetry without impacting the topology.

Radial symmetry will be ignored.

If the topology can't be kept because it is not considered symmetrical, you'll get the option to enforce the mirroring."
symmetry.mirrorLeftToRight "Left to Right"
symmetry.mirrorRightToLeft "Right to Left"
symmetry.mirrorFail "Failed to apply symmetry.

Do you want to enforce symmetry by mirroring the mesh?"
symmetry.mirrorUseMasking "Protect masked area"
symmetry.mirrorUseMasking.help "Keep masked area intact.

This option will be ignored with non-symmetric topology (or disconnected surface, like a pair of eyes)."
// reset
symmetry.reset "Reset"
symmetry.resetCenterMesh "Mesh center"
symmetry.resetCenterWorld "World center"
symmetry.resetDirection "Orientation"
// advanced
symmetry.showLine "Show line"
symmetry.showPlane "Show plane"
symmetry.editWarning "Symmetry edit is experimental."
symmetry.edit "Gizmo edit"
symmetry.edit.help "You can freely set the symmetry plane.

This feature is a bit experimental and you should probably never use it."

// --------------------------------------------------------------------------------------
// tools icons on the left (ICON FIT)
tool.dynTopo "DynTopo"
tool.symmetry "Sym"
tool.mirror "Mirror"
tool.clay "Clay"
tool.clay.sub "Sub"
tool.brush "Brush"
tool.move "Move"
tool.move.normal "Normal"
tool.drag "Drag"
tool.smooth "Smooth"
tool.smooth.relax "Relax"
tool.mask "Mask"
tool.mask.unmask "Unmask"
tool.maskSelector "SelMask"
tool.smudge "Smudge"
tool.flatten "Flatten"
tool.flatten.fill "Fill"
tool.layer "Layer"
tool.crease "Crease"
tool.trim "Trim"
tool.split "Split"
tool.project "Project"
tool.inflate "Inflate"
tool.pinch "Pinch"
tool.nudge "Nudge"
tool.stamp "Stamp"
tool.clearLayer "DelLayer"
tool.gizmo "Gizmo"
tool.gizmo.auto "Auto"
tool.gizmo.editPivot "Pivot"
tool.gizmo.rotateSnap "Snap"
tool.gizmo.local "Local"
tool.transform "Transform"
tool.transform.move "Move"
tool.transform.rotate "Rotate"
tool.transform.scale "Scale"
tool.transform.snap "Snap"
tool.measure "Measure"
tool.view "View"
tool.lathe "Lathe"
tool.tube "Tube"
tool.insert "Insert"
tool.shape.flip "Flip"
tool.shape.view "View"
tool.shape.lasso "Lasso"
tool.shape.curve "Curve"
tool.shape.polygon "Polygon"
tool.shape.path "Path"
tool.shape.rectangle "Rect"
tool.shape.ellipse "Ellipse"
tool.shape.line "Line"
tool.shape.closed "Closed"

// popup when editing sliders
tool.sliderRadius "Radius $0"
tool.sliderIntensity "Intensity $0 %"

// --------------------------------------------------------------------------------------
// title
tool.settingsTitle "Settings ($0)"

// --------------------------------------------------------------------------------------
// tool menu
tool.noSettings "This tool doesn't have any specific settings."

// --------------------------------------------------------------------------------------
// clay
tool.clay.flattenOffset "Flatten offset"

// --------------------------------------------------------------------------------------
// crease
tool.crease.pinchFactor "Pinch force"

// --------------------------------------------------------------------------------------
// layer
tool.layer.removeInfluence "Use current layer offset"
tool.layer.removeInfluence.help "This option is only active when there is a current layer selected.

It will use the current layer offset to limit the displacement over strokes."
tool.layer.noLayerSelected "This option is only available if a current layer is selected"

// --------------------------------------------------------------------------------------
// flatten
tool.flatten.warning "These options are experimental and could be removed in the future!"
tool.flatten.planeLockOrigin "Lock plane origin"
tool.flatten.planeLockNormal "Lock plane direction"
tool.flatten.planeAverageOrigin "Average plane origin"
tool.flatten.planeAverageNormal "Average plane direction"
tool.flatten.planeOffset "Plane offset"

// --------------------------------------------------------------------------------------
// smooth
tool.smooth.stickyBorder "Sticky vertex on border"

// --------------------------------------------------------------------------------------
// paint
tool.paint "Paint"
tool.paint.erase "Erase"
tool.paint.depthFilter "Depth filtering"
tool.paint.layerFilter "Layer filtering"
tool.paint.layerFilter.help "Use this option if you only want to repaint the already painted area of a layer."

// --------------------------------------------------------------------------------------
// masking
tool.mask.clear "Clear"
tool.mask.invert "Invert"
tool.mask.flipConnected "Flip connected"
tool.mask.blur "Blur"
tool.mask.sharpen "Sharpen"
tool.mask.thickness "Shell thickness"
tool.mask.polish "Border smoothness"
tool.mask.engraveEmboss "Engrave / Emboss"
tool.mask.extract "Extract"
tool.mask.split "Split"
tool.mask.closeMask "Closing action (masked):"
tool.mask.closeUnmask "Closing action (unmasked):"
tool.mask.closeAction "Closing action:"
tool.mask.closeActionNone "None"
tool.mask.closeActionFill "Fill"
tool.mask.closeActionShell "Shell"
tool.mask.closeActionLayer "Layer"
tool.mask.closeAction.help "-- None
Simply extract the part and let the extracted part opened.

-- Fill
Hole is filled and smoothed.
Do not use this option for flat surface.

-- Shell
Close the extracted shape by using the thickness value.

-- Layer
Extract the layer difference (layer sub-menu only)."

// --------------------------------------------------------------------------------------
// matrix (transform / gizmo)
tool.matrix "Matrix"
tool.matrix.clone "Clone"
tool.matrix.action "Action"
tool.matrix.action.help "-- Move origin
Move the mesh to the world origin.

-- Reset
Reset the mesh transform to identity.

-- Bake
Apply the matrix to the vertex and reset the matrix. Visually, nothing should change."
tool.matrix.transformOperation "Transform operation"
tool.matrix.translation "Translation"
tool.matrix.rotation "Rotation"
tool.matrix.scale "Scale"
tool.matrix.uniformScale "Uniform scale"
tool.matrix.uniformScale.help "Nomad cannot support non-uniform scale as object transform, so it will be applied as a vertex transformation."
tool.matrix.moveToOrigin "Move origin"
tool.matrix.resetTransform "Reset"
tool.matrix.bakeTransform "Bake"
tool.matrix.applyMethod "Method:"
tool.matrix.applyMethodAuto "Auto"
tool.matrix.applyMethodVertex "Vertex"
tool.matrix.applyMethodObject "Object"
tool.matrix.applyMethod.help "-- Auto
Let Nomad choose between Vertex or Object mode.
Typically, object is preferred unless symmetry is enabled or if there is masking on the mesh.

-- Vertex
Vertices are transformed individually.
Symmetry and mask are taken into account.
For primitives that are not validated, Object mode is forced.

-- Object
The object is transformed as a whole.
Symmetry and mask are ignored.
If you use non-uniform scaling, Vertex mode will be forced."

// --------------------------------------------------------------------------------------
// transform
tool.transform.multiTouch "Multi-touch"
tool.transform.multiTouch.help "If this option is disabled, you can only use one mode (translate, rotate, scale) at a time."

// --------------------------------------------------------------------------------------
// gizmo
tool.gizmo.size "Widget size"
tool.gizmo.linearRollThreshold "Tangent roll threshold"
tool.gizmo.linearRollThreshold.help "Angle threshold to choose between linear or circular roll method.

Value above this threshold will use the circular roll.

If you prefer the linear roll (direction of the tangent), simply set this value to 90°."
tool.gizmo.autoHide "Hide on interaction"
tool.gizmo.tap "Move custom pivot on single-tap"
tool.gizmo.tap.help "This option is only effective in custom pivot mode (Auto disabled).

-- None
Nothing happen when tapping on the mesh.

-- First hit
Move the gizmo on the first intersection.

-- Middle stab
Move the gizmo on the average of the first two intersections."
tool.gizmo.tapNone "None"
tool.gizmo.tapFirstHit "First hit"
tool.gizmo.tapMiddleStab "Middle stab"

// --------------------------------------------------------------------------------------
// lathe
tool.lathe.axis "Axis"
tool.lathe.axis.fixed "Fixed"
tool.lathe.axis.dynamic "Dynamic"

// --------------------------------------------------------------------------------------
// tube
tool.tube.snap "Snapping"
tool.tube.snap.all "Every point"
tool.tube.snap.startEnd "Start & End"

// --------------------------------------------------------------------------------------
// trim
tool.hole "Hole filling"
tool.hole.fillHoles "Fill holes"
// tool.hole.reproject "Reproject filled holes"
// tool.hole.reproject.help "Try to reproject the filled hole so that it follows more closely the cut.
// However, it will only work for rather simple projection."
tool.hole.bridges "Screen-space boolean"
tool.hole.bridges.help "If this option is enabled, you can punch holes in the volume.
The cut slope will also follow more closely the cutting shape."
tool.hole.threshold "Threshold epsilon"
tool.hole.threshold.help "Tweaking this value might help with the hole filling algorithm."
tool.hole.smoothing "Hole smoothing"

// --------------------------------------------------------------------------------------
// smudge
tool.smudge.quality "Quality"
tool.smudge.quality.help "It changes the resolution of the projected pixels, lower values means faster strokes."

// --------------------------------------------------------------------------------------
// trim / split / project / selMask
tool.shape "Shape"
tool.shape.rectangleSquare "Square"
tool.shape.rectangleCentered "Centered"
tool.shape.ellipseCircle "Circle"
tool.shape.ellipseCentered "Centered"
tool.shape.lineRotateStep "Rotate step"

// --------------------------------------------------------------------------------------
// measure
tool.measure.goldenRatio "Show golden ratio"

// --------------------------------------------------------------------------------------
// topology
topology "Topology"
// multires
topology.multires.title "Multiresolution"
topology.multires.title.help "Keep multiple resolution of a mesh.

If you make changes in a lower resolution, details from the higher resolutions will be reprojected when you switch back.

Layers are available on every resolution."
topology.multiresReverse "Reverse"
topology.multiresReverse.confirm "Could not create base subdivision.

The current topology is probably not a result from a subdivision."
topology.multiresSubdivide "Subdivide"
topology.multiresSubdivideConfirm "The mesh will have $0M vertices, are you sure?"
topology.multiresDeleteLower "Delete lower"
topology.multiresDeleteHigher "Delete higher"
topology.multiresKeepTriangles "Keep triangles"
topology.multiresLinear "Flat subdivision"
// voxel
topology.voxel.title "Voxel remeshing"
topology.voxel.title.help "Remeshing by sampling the mesh on a grid.

If the object is not closed (watertight), an hole-filling algorithm will be applied first.

Layers are reprojected after remeshing but the quality will degrade."
topology.voxelResolution "Resolution"
topology.voxelRemesh "Remesh"
topology.voxelSharp "Keep sharp edges"
topology.voxelSharp.help "This option is mostly useful for simple primitive boolean operation.

It will introduce distortion in some area due to the points being snapped on the edges."
topology.voxelSubLevel "Rebuild multires"
topology.voxelSubLevel.help "You can rebuild a multiresolution hierarchy from the voxel remesher output.

It will also run faster and use less memory, especially if the voxel detail value is high.
However if the voxel detail value is low and you are asking for lot of multires level, you'll lose details."
// dynamic topology
topology.surfaceUniform "Remesh"
topology.surfaceDetail "Detail"
topology.surfaceDetail.help "Unlike voxel remeshing, surface remeshing doesn't require the mesh to be closed.

It can also support masking so that you can protect some part of the mesh from topology changes.

Layers are updated correctly."
topology.surfaceMethod "Method"
toplogy.surfaceMethodUniformisation "Uniformisation"
toplogy.surfaceMethodSubdivision "Subdivision"
toplogy.surfaceMethodDecimation "Decimation"
topology.surfaceMethod.help "Behavior of dynamic topology:
- Uniformisation: add and remove detail
- Subdivision: add detail
- Decimation: remove detail"
topology.surfaceUseMasking "Protect masked area"
topology.surfaceUseMasking.help "The masked areas will protect the topology from beging changed."
topology.surfaceExtrapolate "Vertex extrapolation"
// dynamic
topology.dynamic "Dynamic topology"
topology.dynamicActivate "Enabled"
topology.dynamicActivate.help "With dynamic topology, sculpting tools can subdivide or simplify the mesh locally in real time.

This feature can have a noticeable impact on performance.

Layers are updated correctly."
topology.dynamicDetailLevel "Detail"
topology.dynamicDetailEdge "Detail"
topology.dynamicDetailMethod "Level of detail based on..."
topology.dynamicDetailMethodZoom "Zoom"
topology.dynamicDetailMethodRadius "Radius"
topology.dynamicDetailMethodConstant "Constant"
topology.dynamicDetailMethod.help "-- Zoom
The level of detail is based on how far you are from the surface.

-- Radius
The tool radius defines the amount of detail.

-- Constant
The detail is fixed, the detail value is shared with the voxel slider as well."
topology.dynamicQuality "Prefer..."
topology.dynamicQuality.help "If you choose Quality, the 2 main differences are:
- refinement is applied before the sculpting operator, you will get less interpolating artefact when painting or sculpting very small details
- refinement is not applied incrementally, if you sculpt very small details or do quick strokes, the topology will always be correctly refined

For better performance, and if you plan on using this option, you might consider enabling the \"partial drawing\" option in the Settings panel."
topology.dynamicQualitySpeed "Speed"
topology.dynamicQualityQuality "Quality"
topology.dynamicUsePressure "Use pressure on radius"
topology.dynamicUsePressure.help "Use this option if you want the pen pressure impact on tool radius to impact the level of detail."
// topology.dynamicBrush "Brush"
// topology.dynamicGlobal "Global"
// topology.dynamicSettings "Settings - Brush / Global"
// decimate
topology.decimate.title "Decimation"
topology.decimate.title.help "Reduce the number of polygons by trying to keep as much details as possible.

This feature can be useful if you want to export for 3d printing.
However you should probably not use if you want to continue sculpting on it, as it can produce uneven triangles.

Note that masked area won't be decimated."
topology.decimate "Decimate"
topology.decimateTargetFaces "Target triangles"
topology.decimatePaintWeight "Preserve painting"
topology.decimatePaintWeight.help "Higher value will try to preserve painting.

Set this value to 0 if you don't care about the painting."
topology.decimateUniform "Uniform faces"
topology.decimateUniform.help "Higher value will output triangles with similar size."
// topology.decimatePreserveBorders "Preserve borders"
// topology.decimatePreserveBorders.help "Do not decimate the border of the mesh.

// This is only relevant for object that are not watertight."

// BFF is activated through Debug menu
topology.uv.title "UV Auto-Unwrap"
topology.uvAtlas "Unwrap Atlas"
topology.uvAtlas.warning "Kann sehr langsam sein, Ziel: <100k Vertices!"
topology.uvBFF "Unwrap BFF"
topology.uvBFF.warning "Es kann zu Überschneidungen kommen, wenn das Mesh Handles aufweist!"
topology.uvBFFCones "Cone-Anzahl"
topology.uvBFFCones.help "Ein höherer Wert verringert die Verzerrung bei komplexen Objekten.

Ein höherer Wert bedeutet eine längere Berechnungszeit."
topology.uvDelete "UVs löschen"

// --------------------------------------------------------------------------------------
// version trial
version.buyWeb "Die Web-Version ist nur eine Demo"
version.buyFull "Upgrade auf Vollversion"
version.trialLimit "Test Version:
- Nur 3 UnDo/Redo möglich
- Nur 1 Layer pro Mesh
- Nur 1 aktives Projekt
- Kein Import/Export"
version.restorePurchase "Kauf wiederherstellen"
version.fullFeatures "- UnDo/ReDo unbegrenzt
- Unbegrenzte Layer
- Speichern  & Laden
- Export & Import"