// Comments are sync from the english version, it's not possible
// to have language specific comments at the moment.
language_note ""

// comments with "ICON FIT" should be short, ideally < 10 characters

// When in doubt, leave an empty string, it will fallback to english
// Some terms should probably be left untranslated
// For sure: Voxel, Matcap, DynTopo, PBR, Dyntopo
// Not sure: Roughness/Metalness? Mesh? Sub? tool names? etc

// ----------------------------------------------
// general stuffs

// Popup question, confirm? [yes/cancel] [ok/cancel] [delete/cancel]
confirm "請確認是否操作？"
yes "確認"
ok "好"
delete "刪除"
cancel "取消"

// feature: Auto / Off / On
on "開啟"
off "關閉"
auto "自動"

// coordinate
X "X"
Y "Y"
Z "Z"

advancedSettings "高級設置"

// generic warning when there is no mesh selected
noSelectedMesh "對象未選擇"

// generic warning when only one mesh needs to be selected
multipleObjectWarning "您選擇了多個對象，無法修改圖層。"

// ----------------------------------------------

// when you launch the app and there is missing Nomad/data files
loading.reprocess "正在載入數據，請等待... ($0/$1)

$2"

// main pbr channel
baseColor "顏色"
roughness "粗糙度"
metalness "金屬強度"

// ----------------------------------------------
// about
about.minify "全屏顯示"
about.minify.help "在設備支持的情況下，可以通過四指觸碰屏幕來開關全屏顯示"
about.turntable "旋轉展示"
about.turntableSpeed "旋轉速度"
about.credits "鳴謝"
about.creditsOpenSource "開源項目"
about.creditsArts "MatCap與HDRI"
about.languages "多語言切換"
about.languages.help "如您希望了解更多，請訪問https://github.com/stephomi/nomad-translation（英文）"
about.openUrl "是否打開此鏈接？"
// nomad
about.website "官方網站"
about.forum "交流論壇"
about.manual "使用指引"
about.mail "聯系郵箱"
// social
about.twitter "Twitter"
about.instagram "Instagram"
about.facebook "Facebook"
about.discord "Discord"

// ----------------------------------------------
// alert
alert.hole.nothing "該對象沒有孔洞！"
alert.shape.notVisible "當前對象不可見！"
alert.trim.nothing "未找到可裁切對象"
alert.trim.full  "對象不能完全裁切"
alert.mask.noExtract "未找到可提取對象"
alert.mask.noSplit "未找到可分離對象"
alert.view.disabled "一些功能將在瀏覽模式下禁用："
alert.view.disabled.widgetPrimitive "編輯錨點"
alert.separate.fail "該對象只有一部分，所以無法分開"
alert.voxelRemesh.success "網格重構成功！"
alert.voxelRemesh.empty "網格重構失敗，因為結果並未產生面。"
alert.voxelRemesh.invalidInput "輸入無效！"
alert.matrix.clone "將覆制此對象"
alert.gizmo.usePivot "使用自定義坐標原點"
alert.gizmo.useAuto "使用自動坐標原點"
alert.gizmo.editPivot "編輯坐標原點模式"
alert.gizmo.editObject "編輯對象模式"
alert.dynamic.enable "啟用動態網格"
alert.dynamic.disable "關閉動態網格"
alert.colorPicker "在對象上拖動手指選取一個顏色"
alert.backgroundTransform "輕點即可退出背景變換"
alert.camera.resetView "重置視圖"
alert.camera.snapView "切換視圖"
alert.mask.show "顯示蒙版"
alert.mask.hide "隱藏蒙版"
alert.selection.lock "鎖定所選項"
alert.selection.unlock "解鎖所選項"
alert.selection.isolate "隔離所選項"
alert.selection.showAll "顯示全部"
alert.quickSave "正在自動保存..."
alert.forceShowPainting.fill "顯示圖層繪畫已打開"
alert.forceShowPainting.tool "顯示圖層繪畫已打開"
alert.multiresLost "模型細分將會丟失！"
alert.rangeWarning "過高的分辨率將會占用大量內存！"
// autosave popup
alert.autoSave.auto "將在 $0s 後自動保存"
// bottom warning
alert.warning.needLayer "當前工具僅在活動圖層上可用"
alert.warning.multiresLost "模型細分將會丟失！"
alert.warning.paintingHidden "繪畫已被隱藏，請在設置面板里將其打開。"
alert.warning.noPartialWireframe "打開線框顯示時，局部雕刻將被禁用。"
// bottom tip
alert.tip.shapeOrthographic "為了避免因透視視圖而產生的偏差，建議在相機設置里切換到正交視圖。"
// undo
alert.state.trial "這是試用版本，您無法再撤銷。"

// ----------------------------------------------
// background
background "背景"
background.settings "設置" // unused
background.color "顏色" // unused
background.environment "環境"
background.blur "模糊"
background.exposure "曝光"

background.imageEnable "參考圖像"
background.imageOverlay "對象不透明度"
background.imageAlpha "背景不透明度"
background.imageReset "重設"
background.imageTransform "背景變換"
// transform
background.imageX "X軸方向"
background.imageY "Y軸方向"
background.imageRotation "旋轉"
background.imageScale "縮放"

// ----------------------------------------------
// camera
camera "相機"
// saved views
camera.updateView "更新視圖？"
camera.addView "添加視圖"
camera.focusOn "正在觀察"
// projection
camera.projection "視圖"
camera.orthographic "正交視圖"
camera.perspective "透視視圖"
camera.fov "視場角"
camera.focal "35毫米等值焦距：$0mm"
// orbit
camera.orbit "視圖旋轉"
camera.orbit.help "在旋轉模式下，可使用雙指旋轉場景的水平面。"
camera.trackball "旋轉模式"
camera.turntable "水平模式"
// speed
camera.speed "相機速度"
camera.rotation "旋轉"
camera.panning "平移"
camera.zooming "縮放"
// misc
camera.resetView "重置視圖"
camera.snapView "固定視圖"
// interaction
camera.pivot "視圖中心點"
camera.doubleTapMesh "雙擊對象"
camera.doubleTapBackground "雙擊背景"
camera.doubleTapPivot "雙擊後改變"
camera.doubleTapPivot.help "雙擊後改變坐標視圖中心點。"
camera.autoPivot "平移/縮放後改變"
camera.autoPivot.help "雙指移動相機後，中心點會隨之移動。"
camera.doubleTapFocus "聚焦"
camera.doubleTapFocus.help "雙擊物體表面後視圖中心將移動至該點。"
camera.doubleTapFocusSelection "聚焦所選項"
camera.doubleTapFocusSelection.help "雙擊背景後相機將會縮放移動至最適合該對象的視圖。"

// scene and layer lists
curve.preset "預設"
curve.custom "自定義"

// ----------------------------------------------
// debug
debug.uvPrimitive.warning ""
debug.uvPrimitive ""
debug.uvPrimitive.help ""
debug.uvNormalize ""
debug.uvNormalize.help ""
debug.uvBFF ""
debug.uvBFF.help ""
debug.logs ""
debug.heightmap ""
debug.graphics ""
debug.thumbnails ""

// scene and layer lists
expandList "展開圖標"
expandList.help "可以讓菜單里的圖標排列間距放大。"

// ----------------------------------------------
// file
file.project.empty "您沒有保存的項目"
file.project.unsaved "更改未保存！"
file.project.loseUnsaved "如不保存，您的更改將會丟失！"
file.project.lastManualSave "上一次手動保存的預覽"
file.project.trialNoOpen "這是試用版本，您將無法重新打開當前項目！"
file.project.trialOnlyOpen "這是試用版本，您只能打開當前項目！"

// ----------------------------------------------
// project
file.project "項目"
file.project.save "保存"
file.project.save.confirm "確認保存 $0？"
file.project.saveAs "另存為"
file.project.saveAs.confirm "確認覆蓋 $0？"
file.project.open "打開"
file.project.open.confirm "將打開選定的項目 $0？"
file.project.add "添加"
file.project.add.confirm "確認添加 $0 至當前項目？"
file.project.new "新建"
file.project.new.confirm "確認新建場景？"
file.project.delete "刪除"
file.project.delete.confirm "確認刪除 $0？"
file.project.delete.confirmActive "刪除 $0？

這是當前正打開的項目！"
file.project.delete.confirmOk "確定要刪除嗎？"
file.project.rename "重命名"

// autosave
file.project.autoSave "自動保存項目"
file.project.autoSave.confirm "確定要關閉自動保存嗎？"
file.project.autoSave.help "每隔一段時間將您的項目另存為一個單獨的文件。

這個自動保存文件可以在以下目錄找到：

$0"
file.project.autoSave.popup "彈窗提醒"
file.project.autoSave.minutes "自動保存間隔"
file.project.autoSave.delete "刪除自動保存文件"
file.project.autoSave.delete.confirm "確認刪除？"

// import
file.import.title "從外部導入"
file.import.title.help "支持導入的格式：
- Wavefront (.obj)
- glTF 2.0 (.glb .gltf)
- STL (.stl)"
file.importOpen "導入新文件"
file.importOpen.confirm "確定導入新文件？"
file.import.add "添加到場景"
file.import.add.confirm "確定添加新文件？"

file.exportSelection "只導出選擇部分"
file.exportSelection.help "只導出當前選擇對象，而不是所有場景。"
file.convertToQuad ""
file.convertToQuad.help ""

// export
file.export.title "導出"
file.export.title.help "建議導出 glTF 格式，因為它比其他格式支持更多屬性。"

// generic export
file.export.texture ""
file.export.texture.help ""
file.export.normal "導出法線"
file.export.normal.help "如想在其他軟件上打開該文件，請勾選此選項。
該選項對本應用沒有影響。"

// gltf
file.export.gltf "導出 glTF 2.0"
file.export.gltfLayer "導出圖層"
file.export.gltfLayer.help "將圖層導出為可變體。這是來自官方的特性，能在更多軟件上使用。"
file.export.gltfColor "導出顏色"
file.export.gltfColor.help "導出的是頂點顏色。這是來自官方的特性，能在更多軟件上使用。"
file.export.gltfExtraPaint "導出其他"
file.export.gltfExtraPaint.help "將導出粗糙度、金屬強度、蒙版和圖層繪畫。其他軟件不會讀取該屬性。"

// obj
file.export.obj "導出 OBJ 格式"
file.export.objWarning "圖層、粗糙度、金屬強度、蒙版和繪畫圖層等其他屬性將會丟失。"
file.export.objColorAppend "導出顏色"
file.export.objColorAppend.help "給頂點添加顏色信息。

只有部分3D軟件能夠識別。"
file.export.objColorHexa "十六進制顏色"
file.export.objColorHexa.help "像ZBrush那樣將顏色轉換為十六進制。

該屬性只有部分3D軟件能夠識別。"

// stl
file.export.stl "導出 STL 格式"
file.export.stlWarning "圖層、粗糙度、金屬強度、蒙版和繪畫圖層等其他屬性將會丟失。"
file.export.stlColor "導出顏色"
file.export.stlColor.help "只有部分3D軟件能夠識別。"
file.export.stlAscii "默認情況下，格式為二進制。

您可以選擇導出為文本格式（ASCII），但文件會更大。"

file.settings.title "設置"
file.settings.title.help "大部分應用的設置都保存在此處（相機界面等）。

某些資源將自動保存在其他地方，包括：
- HDRI
- 材質
- 畫筆形狀
- 背景
- 項目
"

// settings
file.settings.reset "恢覆默認設置"
file.settings.reset.confirm "確定重設所有設置？

項目、畫筆形狀、材質、HDRI與背景將不會被影響。"

// materials
file.materials "材質庫"
file.materials.reset "重置為默認"
file.materials.reset.confirm "確定要重置材質庫嗎？"

// tools
file.tools "工具預設"
file.tools.reset "重置為默認"
file.tools.reset.confirm "確定要重置材質庫嗎？"

// render
file.render "渲染"
file.render.showInterface "顯示操作界面"
file.render.renderRatio ""
file.render.renderRatio.help ""
file.render.help ""
file.render.size "渲染尺寸"
file.render.size.custom "自定"
file.render.screenResolution "屏幕尺寸"
file.render.export "導出為png"
file.render.width "寬度"
file.render.height "高度"
file.render.warn "您當前導出的圖片尺寸較大（$0x$1）!

請確認文件妥善保存後再繼續導出，否則程序可能會因運存占用過高而閃退。"
file.render.transparent "導出透明背景"
file.render.transparent.help "打開此選項可以讓您更方便地把渲染圖導入到平面軟件里。

暫不支持部分透明對象導出。"

// ----------------------------------------------
// gesture menu
gesture.useGlobal "使用全局壓感設置"
gesture.useGlobal.help "勾選後，所有工具都會使用相同的壓感參數。

如您希望給此工具單獨設定壓感參數，請取消勾選。"

gesture.pressure "壓感"
gesture.pressureTitle "壓感設置 ($0)"
gesture.pressure.noTool "此工具不適用壓感設置。"
gesture.pressure.noGrab "此工具會忽略壓感設置。"
gesture.pressure.radius "半徑"
gesture.pressure.intensity "強度"
gesture.pressure.useRadius "啟用"
gesture.pressure.useIntensity "啟用"
gesture.pressure.curveRadius "半徑"
gesture.pressure.curveIntensity "強度"

gesture.cameraInteraction "相機移動"
gesture.sculptInteraction "雕刻"
gesture.interaction.fingerAndStylus "手指與觸控筆"
gesture.interaction.finger "手指"
gesture.interaction.stylus "觸控筆"

gesture.fingerLighting "三指旋轉燈光"
gesture.fingerLighting.help "在屏幕上使用三指水平移動可使環境、燈光與材質捕捉旋轉。"
gesture.fingerRadius ""
gesture.fingerRadius.help ""

gesture.fingerSmooth "將手指用於平滑"
gesture.unknownPressure "允許未識別的壓感"
gesture.unknownPressure.help "當您的觸控筆壓感無法使用或者希望使用手指壓感時，請勾選此選項。"

// pencil
gesture.pencilAction.none "無"
gesture.pencilAction.smooth "平滑"
gesture.pencilAction.alt "添加或減去"
gesture.pencilAction.android "觸控筆按鈕"
gesture.pencilAction.android.help "實驗功能"
gesture.pencilAction.ios "雙擊Pencil"
gesture.pencilAction.ios.help "僅支持Apple Pencil 第二代"

// history
gesture.history "快捷手勢"
gesture.history.help "雙指輕點撤銷。

三指輕點重做。"

// size rejection
gesture.useSizeRejection "啟用忽略尺寸"
gesture.useSizeRejectionConfirm ""
gesture.useSizeRejection.help "如果手指與屏幕的接觸面積超過設定值，屏幕將忽略手指的本次操作。

部分設備可能不支持此選項"
gesture.sizeRejection "尺寸閾值"
// help
gesture.interaction.title "防誤觸"
gesture.interaction.title.help "以下選項均為全局設置。"

// ----------------------------------------------
// history
history "歷史記錄"
history.root "新建"
history.undoConfirm "您確定要撤銷所有操作嗎？"
history.undoWarning "如在此之後進行更改，將會覆蓋之前的所有操作。"
history.stack "歷史記錄設置"
history.limitSize "歷史記錄限制 (Mb)"
history.limitSize.help "歷史記錄的最大大小（以Mb為單位）。

歷史記錄狀態會隨著下一個操作記錄而改變。"
history.limitStack "歷史記錄步數"
history.limitStack.help "程序可保留的最大操作數量。

歷史記錄狀態會隨著下一個操作記錄而改變。"
history.rangeProtect "歷史記錄保護範圍"
history.rangeProtect.help "如您在歷史記錄中做了大量操作，程序會在覆蓋操作之前提示您。"
history.restoreCamera "恢覆相機視角"
history.restoreCamera.help "啟用該選項後您可以在撤銷或重做時同時恢覆當時的相機視角。"
// display undo/redo
history.state.undo "撤銷： $0"
history.state.redo "重做： $0"
history.state.symmetrySplit "鏡像"
history.state.voxelRemesh "體素網格重構"
history.state.surfaceRemesh "表面網格重構"
// state multires
history.state.multiresToDynamic "模型細分轉為動態網格"
history.state.multiresLevel "改變分辨率"
history.state.multiresSubdivide "細分網格"
history.state.multiresReverse "簡化網格"
history.state.multiresDeleteLower "刪除低分辨率模型"
history.state.multiresDeleteHigher "刪除高分辨率模型"
// mesh
history.state.meshDynamicToStatic "動態網格轉為靜態網格"
history.state.meshStaticToDynamic "靜態網格轉為動態網格"
history.state.meshSymmetryUpdate "改變對稱"
history.state.meshMatrixUpdate "軸向變換"
history.state.meshVisibility "可見性"
history.state.meshMaterial "改變材質"
// state scene
history.state.sceneAddRemove "場景"
history.state.sceneMeshOrder "模型順序"
// state layer
history.state.layerOrder "更改圖層順序 $0"
history.state.layerMergeRedo "取消合並圖層 $0"
history.state.layerCreate "添加圖層 $0"
history.state.layerDelete "刪除圖層 $0"
history.state.layerMerge "合並圖層 $0"
history.state.layerHide "隱藏圖層 $0"
history.state.layerShow "顯示圖層 $0"
history.state.layerSelect "選擇圖層 $0"
history.state.layerUnselect "取消選擇圖層 $0"
history.state.layerName "圖層 $0 重命名"
history.state.layerFactor "調整圖層 $0 參數"
history.state.layerFactorOffset "調整圖層 $0 偏移度"
history.state.layerFactorColor "調整圖層 $0 透明度"
history.state.layerFactorRoughness "調整圖層 $0 粗糙度"
history.state.layerFactorMetalness "調整圖層 $0 金屬強度"
// state light
history.state.lightVisible "調整燈光 $0 可見性"
history.state.lightIntensity "調整燈光 $0 強度"
history.state.lightColor "調整燈光 $0 顏色"
history.state.lightPosition "調整燈光 $0 位置"
history.state.lightShadow "調整燈光 $0 陰影"
history.state.lightBias "調整燈光 $0 陰影偏移"
history.state.lightAttachment "調整燈光 $0 定位方式"
history.state.lightAdd "添加燈光 light $0"
history.state.lightDelete "刪除燈光 $0"
history.state.lightCopy "覆制燈光 $0"
history.state.lightMove "移動燈光 $0"
history.state.lightType "改變燈光 $0 類型"
history.state.lightSpotAngle "改變燈光 $0 入射角"
history.state.lightSpotSoftness "改變燈光 $0 硬度"
// state view
history.state.viewAdd ""
history.state.viewMove ""
history.state.viewDelete ""

// ----------------------------------------------
// interface
interface "界面設置"

// bottom buttons
interface.bottomButtons "底部快捷方式"
interface.shortcut.voxelRemesh "體素網格重構"
interface.shortcut.wireframe "網格開關"
interface.shortcut.lockSelection "鎖定選擇"
interface.shortcut.lockSelection.help "啟用後，您無法通過點擊方式選擇對象。"
interface.shortcut.cameraReset "重置視圖"
interface.shortcut.cameraSnap "切換視圖"
interface.shortcut.perspective "透視視圖"
interface.shortcut.cameraSnapFlip "翻轉基本視圖"
interface.shortcut.cameraSnapFlip.help "當相機處於基本視圖時，點擊切換視圖將會翻轉至背面。"

// colors
interface.colors "界面顏色"
interface.colorSelect "主色"
interface.colorBase "底色"
interface.colorBaseTransparent "面板顏色"
interface.panelTransparent "面板透明度"
interface.blurFactor "模糊強度"

// color preset
interface.colorsPresets "界面預設"
interface.presetBlurRed "紅"
interface.presetBlurBlue "藍"
interface.presetBlurGreen "綠"
interface.presetBlurYellow "黃"
interface.presetBlackWhite "黑與白"
interface.presetWhiteBlack "白與黑"
interface.presetLividOrange "青與橙"
interface.presetCardboard "紙板"
interface.presetDefault "默認設置"

// style
interface.style "菜單風格"
interface.resetAll "重置界面設置"
interface.resetAll.confirm "確定要重置界面嗎？"
interface.flipTop "整體反轉"
interface.flipBottom "反轉底部圖標"
interface.flipMiddle "反轉中間圖標"
interface.showTooltips "顯示工具提示"
interface.showTooltips.help "你在點的這個小問號就是工具提示 :-D"
interface.materialPreview "調整材質參數預覽"
interface.toolboxHide "自動隱藏工具欄"
interface.toolboxHide.help "如果您想隱藏工具欄，請勾選此選項。"
interface.toolboxMaxColumn "工具欄列數"
interface.toolboxResetOrder "重設"
interface.rounding "界面圓角"
interface.curveToolSymmetric "衰減曲線對稱"
interface.curveToolSymmetric.help "使筆刷設置里的衰減參數曲線對稱。"
interface.scale "界面縮放"
interface.cursorStep "垂直間距"
interface.panelWidth "面板寬度"
interface.fontScale "字體尺寸"

// ----------------------------------------------
// layer sub menu
layer.action "操作"
layer.name "重命名"
layer.delete "刪除圖層"
layer.move "移動圖層"
layer.duplicate "覆制圖層"
layer.mergeDown "向下合並"
layer.factors "通道參數"
layer.offsetFactor "位置偏移"
layer.colorFactor "顏色濃度"

// ----------------------------------------------
// layers menu
layers.addLayer "添加圖層"
layers.addLayerTrial "試用版本只能給每個對象添加一個圖層！"
layers.title "圖層"
layers.title.help "圖層能夠記錄位置偏移和繪畫，這對於非線性工作流程來說非常有用。
例如，通過試驗不同的面部表情而不依賴於歷史記錄來撤消更改。

圖層是從上到下排序的，所以上方的圖層會遮蓋下方的圖層。

為了解決圖層的不透明性，圖層的所有通道（顏色濃度、粗糙度、金屬強度）都會使用相同的蒙版。
您可以使用橡皮工具來擦除當前圖層上的繪畫蒙版。"
layers.primitive "基本體無法添加圖層。"
layers.baseSelected "無"

// ----------------------------------------------
// light sub menu
light "光線"
light.color "顏色"
light.intensity "強度"
light.attachment "光照方向"
light.attachment.fixed "固定"
light.attachment.camera "隨相機移動"
light.attachment.environment "環境"
light.attachment.help "- 固定
燈光方向不會改變。

- 隨相機移動
燈光方向隨著相機視角而改變。"
light.type "類型"
light.type.directional "平行光"
light.type.spot "聚光燈"
light.type.point "點光源"
light.spotAngle "入射角"
light.spotSoftness "邊緣硬度"
light.shadowCast "顯示陰影"
light.shadowNormalBias "陰影偏差"
light.visible ""
light.resetPosition ""

// ----------------------------------------------
// material
material "材質混合模式"
material.addNew "添加新材質"
// if the shading mode is in matcap or unlit
material.unlitWarning "粗糙度與金屬強度在當前渲染模式下不可用。"
material.unlitReflectanceWarning ""
material.unlitRefractionWarning ""
// refraction
material.ior "折射率"
material.paintingOverride "表面效果"
material.paintingOverride.help ""
material.refractionSurfaceGlossiness "光澤度"
material.refractionSurfaceGlossiness.help "光澤度可以增強光線打在物體表面上的反射效果。

- 當光澤度為0時，對象表面的粗糙度將與材質相同。

- 當光澤度為1時，對象表面的光滑效果將達到最強。"
material.refractionInteriorRoughness "粗糙度"
material.refractionInteriorRoughness.help "粗糙度可以增強光線透過物體後的散射效果。

- 當粗糙度為0時，對象內部的粗糙度將與材質相同。

- 當粗糙度為1時，對象內部的粗糙效果將達到最強。"
material.paintGlossy ""
material.paintGlossy.help ""
// absorption
material.absorptionEnable ""
material.absorptionEnable.help ""
material.absorptionFactor ""
// alpha
material.opacity "不透明度"
material.alphaMode.opaque "實心"
material.alphaMode.blending "正常混合"
material.alphaMode.blending.help ""
material.alphaMode.additive "線性減淡"
material.alphaMode.additive.help ""
material.alphaMode.dithering "抖動"
material.alphaMode.dithering.help "加入一些噪點使顏色過渡更為平滑。"
material.alphaMode.refraction "折射"
material.alphaMode.refraction.help ""
// shadows
material.castShadows "投射陰影"
material.receiveShadows "接收陰影"
// backface
material.twoSided "雙面折射"
material.alwaysUnlit "不受光顯示"
material.flipCulling "翻轉法線"
// material
material.reflectance ""
material.reflectance.help ""

// ----------------------------------------------
// menu name (visible on small screen menu are collapsed)
menu.files "文件"
menu.scene "場景"
menu.multires "多重網格"
menu.voxel "重構"
menu.dyntopo "動態網格"
menu.topology "拓撲"
menu.primitive "基本體"
menu.render "渲染"
menu.material "材質"
menu.postProcess "後期處理"
menu.camera "相機"
menu.background "背景"
menu.tool "工具"
menu.stroke "筆刷設置"
menu.paint "畫筆設置"
menu.symmetry "對稱"
menu.pressure "壓感"
menu.layers "圖層"
menu.settings "顯示設置"
menu.interface "界面設置"
menu.history "歷史記錄"
menu.historySettings "歷史記錄設置"
menu.about "關於"
menu.debug ""

// ----------------------------------------------
// mesh sub menu
mesh.action "操作"
mesh.holeClose "封閉孔洞"
mesh.holeDetail "分辨率"
mesh.separate "分離對象"
mesh.triplanarWarning "圖層、繪畫與模型細分將會丟失。"
mesh.triplanarResolution "分辨率"
mesh.triplanarCubic "強制轉換為立方體"
mesh.triplanarConvert "轉換"
mesh.name "名稱"
mesh.type "類型"
mesh.typeStatic "靜態模型"
mesh.typeMultiresolution "模型細分"
mesh.typeDynamic "動態模型"

// ----------------------------------------------
// painting
paint.useGlobal "應用全局材料"
paint.useGlobal.help "如勾選此選項，其他工具的材質也將會與所選材質相同。"
paint.usePainting "啟用繪圖"
paint.intensity "畫筆強度"
paint.paintAll "全部上色"
paint.paintAll.help "將當前材料應用到所選對象上。"
paint.paintAllForce "強制全部上色"
paint.paintAllForce.help "將當前材料應用到所選對象上。

蒙版區域與未勾選通道也會被應用。"
paint.strokePaintingTitle "畫筆設置 ($0)"
paint.layerWarning "圖層上的通道蒙版不可用。"
paint.texture.title "貼圖繪制"
paint.texture.title.help "將圖片作為繪畫的筆刷,在模型表面繪畫。

您可以在筆刷的形狀設置里更改貼圖繪制的平鋪、縮放參數。"
paint.texture.warningEnable "您需要先勾選菜單頂部的“啟用繪圖”後才能使用貼圖繪制功能。"
paint.texture.warningIgnored "當前工具無法使用貼圖繪制功能。"
paint.useAlpha "使用筆刷形狀設置"
paint.useAlpha.help "筆刷菜單里的“形狀”設置也會作用到貼圖繪制效果。"
paint.useFalloff "使用筆刷衰減設置"
paint.useFalloff.help "筆刷菜單里的“衰減”設置也會作用到貼圖繪制效果。"

// ----------------------------------------------
// popup (for example tap on a tool, to open edit popup)
popup.save "保存"
popup.save.confirm "確認要保存嗎？"
popup.lastSave ""
popup.lastSave.confirm ""
popup.reset "重置"
popup.reset.confirm "確認要重置嗎？"
popup.clone "克隆"
popup.rename "重命名"
popup.delete "刪除"
popup.delete.confirm "請確認是否刪除？"
popup.delete.confirm.yes "確認刪除"

// title when requesting input value through virtual keyboard
input.name ""
input.number ""
input.hexcolor ""

// ----------------------------------------------
// postprocess
postprocess.mainEnable "後期處理"
postprocess.quality "效果質量"
postprocess.quality.help "該設置可提升部分效果的渲染質量，但可能會影響性能。

僅對以下效果有效：
- 屏幕空間反射（SSR）
- 環境光屏蔽（AO）
- 景深
"
postprocess.maxSamples "最大采樣值"
postprocess.fullResolution "最大實時分辨率"
// fxaa
postprocess.fxaaEnable "快速抗鋸齒（FXAA）"
// taa
postprocess.taaEnable "時間性抗鋸齒（TAA）"
postprocess.taaEnable.help "TAA可減少相機移動時的閃爍，呈現出更加平滑的圖像效果。"
// ssr
postprocess.ssrEnable "屏幕空間反射（SSR）"
postprocess.ssrFactor "強度"
postprocess.ssrDistanceFading "淡化距離"
postprocess.ssrDistanceFading.help "根據反射距離來減弱效果。
此選項能減弱SSR所產生的偽影。"
postprocess.ssrPBRWarning "SSR僅在PBR渲染模式下有效。"
// ssao
postprocess.ssaoEnable "環境光遮蔽（AO）"
postprocess.ssaoRadius "半徑"
postprocess.ssaoFactor "強度"
postprocess.ssaoBias "擴張強度"
postprocess.ssaoBias.help "模型表面曲率將會影響效果的強度。"
// dof
postprocess.dofEnable "景深"
postprocess.dofBlurFar "遠景模糊"
postprocess.dofBlurNear "近景模糊"
postprocess.dofFocusTip "點擊對象表面可以改變焦點。"
// bloom
postprocess.bloomEnable "泛光效果"
postprocess.bloomIntensity "強度"
postprocess.bloomRadius "半徑"
postprocess.bloomRadius.help "此參數可調節泛光的寬度。"
postprocess.bloomThreshold "閾值"
postprocess.bloomThreshold.help "泛光閾值能夠判斷泛光的光線強度。
閾值越高，空間里越亮的物體才會產生泛光效果。"
// tone mapping
postprocess.toneEnable "色調映射"
postprocess.toneExposure "曝光"
postprocess.toneContrast "對比度"
postprocess.toneSaturation "飽和度"
postprocess.toneMappingNone "無"
// curve
postprocess.curveEnable "曲線"
postprocess.curve.luminance "亮度"
postprocess.curve.red "紅"
postprocess.curve.green "綠"
postprocess.curve.blue "藍"
postprocess.curveReset "重設"
postprocess.curveResetAll "全部重設"
// chromatic
postprocess.chromaticEnable "色彩偏移"
postprocess.chromaticFactor "強度"
// vignette
postprocess.vignetteEnable "暈影"
postprocess.vignetteSize "半徑"
postprocess.vignetteHardness "硬度"
// sharpness
postprocess.sharpnessEnable "銳化"
postprocess.sharpnessFactor "強度"
// grain
postprocess.grainEnable "噪點"
postprocess.grainFactor "強度"
// curvature
postprocess.curvatureEnable "曲率描邊"
postprocess.curvatureCavity "縫隙顏色"
postprocess.curvatureBump "凸起顏色"

// ----------------------------------------------
// primitive (scene menu)
primitive "基本體"
primitive.box "立方體"
primitive.sphereCube "球體"
primitive.sphereUV "UV球體"
primitive.icosahedron "寶石"
primitive.cylinder "圓柱"
primitive.cone "圓錐"
primitive.torus "圓環"
primitive.lathe "車削"
primitive.tube "圓管"
primitive.plane "平面"
primitive.triplanar "三向投影"
primitive.needValidate "基本體需轉換為可編輯對象後才可雕刻。"

// for 3d editing in viewport
primitive.useFloatPanel "快捷編輯浮窗"
primitive.useFloatPanel.help "在瀏覽模式下顯示編輯基本體的小浮窗。"
primitive.edit "編輯"
primitive.edit.help ""

primitive.mainConfig "範圍"
primitive.topology "拓撲參數"
primitive.geometry "幾何"

// common config
primitive.validate "轉換"
primitive.maxFaces "面數限制"
primitive.maxFaces.help "該基本體可擁有的最大面數。

該參數只能在轉換為可編輯對象前修改。"
primitive.linear "線性細分"
primitive.subdivision "細分等級"

// common config
primitive.radius "半徑尺寸"
primitive.size "尺寸"
primitive.sizeX "尺寸 X"
primitive.sizeY "尺寸 Y"
primitive.sizeZ "尺寸 Z"
primitive.division "分段數"
primitive.divisionX "分段數 X"
primitive.divisionY "分段數 Y"
primitive.divisionZ "分段數 Z"
primitive.angleX "角度 X"
primitive.angleY "角度 Y"
primitive.angleZ "角度 Z"
primitive.constantDensity "固定比例"
primitive.projectOnSphere "投影在球體上"
primitive.projectOnSphere.help "將點分布在一個完美的球體上。"

// triplanar
primitive.triplanar.title "三向投影 - 設置"
primitive.triplanar.title.help "三向投影是將原對象三個平面的投影重新組合起來填充體素網格，然後將其多邊形化。

您可以通過在三個投影上修改蒙版或移動滑塊的方式來改變幾何體。

建議您關閉對稱選項，否則可能會導致最終效果與您的預期不符。

您可以開啟遮罩面板中的“拓撲連接”選項來在繪制時影響其他平面。"
primitive.triplanarSameSize "固定比例（立方體）"
primitive.triplanarPolish "羽化"
primitive.triplanarResetMask "重置蒙版"
primitive.triplanarReproject "重新投影"
primitive.triplanarReproject.title "修改三向投影設置後會更新平面上的蒙版。

如您不勾選此選項，它將恢覆為默認的球形蒙版。"
primitive.isolate.all "全部"
primitive.isolate.back "背面"
primitive.isolate.right "右面"
primitive.isolate.bottom "底面"
// plane
primitive.planeSameSize "固定比例（平面）"
// box
primitive.boxRegular "固定比例（立方體）"
// tube
primitive.tubeSnapOffset "吸附偏移"
primitive.tubeSnapOffset.help "當偏移值為1.0時，偏移後的距離即為圓管半徑。"
primitive.tubeThicknessStart "起始半徑"
primitive.tubeThicknessEnd "結束半徑"
// primitive.tubeTwist "Twist"
// primitive.tubeTwistRotate "Rotation"
// primitive.tubeTwistRadius "Magnitude"
// primitive.tubeTwistOffset "Offset"
primitive.tubeSnap "吸附"
// lathe
// torus
primitive.torusRadiusOuter "外圈半徑"
primitive.torusRadiusInner "內圈半徑"
primitive.torusAngle "角度"
primitive.torusAngleOffset "角度"
// cylinder
primitive.cylinderHeight "高度"
// cone
primitive.coneRadius "半徑"
primitive.coneHeight "高度"
// hole sub menu (cylinder, tube, etc)
primitive.hole "孔洞"
primitive.hasHole "添加孔洞"
// both used for hole radius and main radius
primitive.radiusSync "相同半徑"
primitive.radiusStart "底面半徑"
primitive.radiusEnd "頂面半徑"
primitive.editRadius "調整半徑"
// spline (for lathe and tube)
primitive.spline "樣條"

// common resources stuffs
resource.delete "刪除"
resource.import "導入"

// ----------------------------------------------
// scene
scene.title "場景"
scene.title.help "當場景內有多個對象時，在覆選框處上下滑動即可快速選擇多個對象。 "
scene.mergeSimple "簡單合並"
scene.mergeVoxel "體素合並"
scene.voxelResolution "分辨率"
scene.subtractionTip "相減運算：隱藏減去對象後點擊體素合並。"
scene.intersectionTip "相交運算：隱藏所有相關模型後點擊體素合並。"

// ----------------------------------------------
// settings
settings.displayTitle "顯示設置"
// wireframe
settings.wireframeTitle "對象網格設置"
settings.wireframeDisplay "對象網格"
settings.wireframeColor "對象網格顏色"
settings.wireframeUV ""
settings.wireframeUV.help ""
settings.debugUV ""
settings.debugUV.help ""
// backface
settings.backfaceTitle "雙面顯示設置"
settings.backfaceVisible "雙面顯示"
settings.backfaceVisible.help "打開雙面顯示可以讓您看到模型的“內面”。

所有的三角形或四邊形都朝向一個特定的方向，例如在基本球體上，所有的面都朝向外部。

如果您將相機移動到球體內部，這些面就是背面。"
settings.backfaceColor "內面顏色"
settings.backfaceColored "內面著色"
// outline
settings.outlineTitle "輪廓"
settings.outlineEnable "被選對象輪廓"
settings.outlineThickness "粗細"
settings.outlineColor "顏色"
// snap cube
settings.snapCubeTitle "方位視圖"
settings.snapCubeDisplay "方位視圖"
settings.snapCubeBottom "置於底部"
settings.snapCubeLeft "置於左側"
// stats
settings.statsTitle "顯示場景狀態"
settings.statsDisplay "顯示場景狀態"
settings.statsRight "置於右側"
settings.statsAll "顯示全部"
// grid
settings.gridTitle "世界網格"
settings.gridDisplay "世界網格"
settings.gridHeight "高度"
settings.gridColor "顏色"
// cursor
settings.cursorWhileSculpting "雕刻時顯示畫筆"
settings.cursorShowDot "顯示指針點"
settings.cursorShowDot.help "指針點會在您移動相機和雕刻時顯示。"
settings.cursorShowRope "顯示畫筆準星"
// other
settings.renderRatio "實時渲染分辨率"
settings.darkenUnselected "變暗未選對象"
settings.smoothShading "平滑著色"
settings.partialDraw "局部雕刻"
settings.partialDraw.help "功能未完善！

僅建議您在雕刻高精度細小模型時使用。

它能讓雕刻過程更加流暢，但不建議您打開顯示對象網格。

啟用此功能可能會在使用畫筆時產生一些奇怪的東西。"
settings.partialDrawWarning "如果顯示不正常，請不要忘記關閉此選項！"
settings.showPainting "圖層繪畫"
settings.lightIcon "燈光圖標"
settings.lightIcon.help "在屏幕上顯示燈光圖標，這樣您可以直接選擇並編輯燈光。"
settings.holeTitle "填補孔洞"
settings.holeNonManifold "填補非流形孔洞"
settings.holeNonManifold.help "應用將會嘗試填補非流形孔洞。

此選項不會被保存在設置中。
"
settings.loadGuiSettings "加載項目GUI設置"
settings.loadGuiSettings.help "當您打開或導入項目文件時，同時加載項目中包含的GUI設置。"
settings.loadObjKeepGroup ""
settings.loadObjKeepGroup.help ""
settings.loadMergeLayers "導入時合並圖層"
settings.loadSkipTextures ""
settings.loadKeepTopology "導入時保留拓撲"
settings.loadKeepTopology.help "如您不希望應用破壞導入模型拓撲，請勾選此選項。

應用將不會：
- 重新排序頂點和面
- 刪除重疊頂點和面
- 移除未使用頂點
"
// multires
settings.multiresTitle "模型細分"
settings.multiresMaxVertices "最大頂點數量"
settings.multiresMaxVertices.help "應用在模型細分之前並不會檢查內存，多邊形數量過多很容易會導致應用崩潰。"
settings.multiresLowResVertices "最低分辨率閾值"
settings.multiresLowResVertices.help "在您移動相機時，模型對象可能會以較低分辨率顯示。

如您希望顯示更高的分辨率，可以增加此值。"
// experimental
settings.experimentalTitle "實驗性功能"
settings.notSaved "這些選項不會在設置中保存。"
// settings.parallel "Parallel sculpting"

// ----------------------------------------------
// shading
shading "渲染模式"
// main render mode
shading.pbr "PBR"
shading.matcap "材質捕捉"
shading.unlit "不受光"
// lights
shading.lights "燈光"
shading.lights.addLight "添加燈光"
shading.lights.unlitWarning "燈光選項在當前渲染模式下不可用。"
// environment
shading.environment "HDRI"
shading.environmentImport "導入HDRI"
shading.environmentExposure "曝光"
shading.environmentBackgroundBlur ""
shading.environmentRotation "旋轉"
shading.environmentRotation.help "在屏幕上使用三指水平移動可使環境、燈光與材質捕捉旋轉。"
shading.environmentAttachedToCamera "固定HDRI"
shading.environmentAttachedToCamera.help "啟用此選項後，移動相機時將不會移動HDRI。

這能讓光線保持固定，對於雕刻來說非常有用。"
// matcap
shading.matcap "材質捕捉"
shading.matcapRotation "旋轉"
shading.matcapRotation.help "在屏幕上使用三指水平移動可使環境、燈光與材質捕捉旋轉。"
shading.matcapGlobal "使用全局材質捕捉"
shading.matcapGlobal.help "如果您希望在不同對象上使用不同的材質捕捉，請取消此選項。"

// ----------------------------------------------
// bottom shortcut buttons (ICON FIT)
shortcut.voxel "重構"
shortcut.wire "網格"
shortcut.mask "蒙版"
shortcut.reset "重設"
shortcut.snap "切換"
shortcut.solo "隔離"
shortcut.lock "鎖定"
shortcut.persp "透視"

// ----------------------------------------------
// stat
stat.ramScene "場景"
stat.vramScene "顯存場景"
stat.vramRender "顯存渲染"
stat.vramTextures ""
stat.ramHistory "歷史"
stat.ramOther "其他"
stat.usedMemory "已用內存"
stat.freeMemory "剩余內存"
stat.ram ""
stat.used "已用：$0 MB"
stat.free "剩余：$0 MB"
stat.faces "面數"
stat.triangles "三角面"
stat.vertices "頂點"
stat.quads "四邊形"
stat.sceneFaces "場景面數"
stat.sceneVertices "場景頂點數"

// ----------------------------------------------
// stroke
stroke "筆刷"
strokeTitle "筆刷設置 ($0)"
stroke.useWorldRadius "恆定筆刷半徑"
stroke.useWorldRadius.help "勾選後筆刷半徑將不會隨著視圖的縮放而改變。

此選項將會影響到所有工具。"
stroke.useShareRadius "相同筆刷半徑"
stroke.useShareRadius.help "使所有工具的筆刷半徑相同。"
stroke.minSpacingAdjustIntensity "調整間隔強度"
stroke.minSpacingAdjustIntensity.help "調整筆刷強度，以保證根據筆畫間距產生一定的變化。"
stroke.minSpacing "筆刷間距"
stroke.minSpacing.help "該選項可調節每個筆畫之間的距離，與筆畫半徑有一定的相關性。

將該值調低可使筆刷顯得更加順滑，但也會影響性能。"
stroke.lazySmooth "平滑筆畫"
stroke.lazySmooth.help "通過平均計算多個點來獲得更加平滑的筆畫。

將該值調高會使筆畫變得不跟手。"
stroke.lazyRadius "筆畫延後"
stroke.lazyRadius.help "筆畫將會按一定的距離延後於指針位置。

此功能可用於繪制平滑線條。"
stroke.globalSettings "這是一個全局設置。"
stroke.snapRadius "續接筆畫範圍"
stroke.snapRadius.help "如果落筆處在最後一筆的續接範圍內，筆刷將會自動續接上。

此功能可用於繪制長線條，但需要頻繁停頓時。"
stroke.sculptOffset "筆刷偏移"
stroke.sculptOffset.help "使筆刷持續偏移於觸控處

此功能適用於小屏幕設備。在使用時，手指不會遮擋到屏幕。"
stroke.accumulate "疊加筆畫"
stroke.accumulate.help "如啟用此選項，則每個筆畫可添加或減去的數量將不會有限制。"
stroke.useDynamicTopology "允許動態網格"
stroke.connectedTopology "連接拓撲"
stroke.connectedTopology.help "啟用此選項後，畫筆將會只雕刻連接到所選表面的頂點。

此選項一般適用於移動工具, 例如專門移動與另一零件自相交的零件。 "
stroke.onlyFrontFace "只影響對象表面"
stroke.onlyFrontFace.help "打開此選項後，應用會忽略對“背面”的操作。

該功能可幫助您在不影響另一側的情況下在幾何平面上繪畫。

該選項也可用於雕刻，但您可能會遇到一些不便。"
stroke.onlySameSide "只移動同向頂點"
stroke.onlySameSide.help "在修改對象造型時，不移動朝向不同的頂點。"
stroke.curveFalloff "衰減"
stroke.onlyLasso "該設置僅對套索工具有效。"
// alpha
stroke.alpha "形狀"
stroke.alphaInvert "形狀反相"
stroke.alphaWrap "平鋪"
stroke.alphaWrap.none "無"
stroke.alphaWrap.repeat "重覆"
stroke.alphaWrap.mirror "鏡像"
stroke.alphaProject "平鋪模式"
stroke.alphaProject.surfaceContinuous "表面連續"
stroke.alphaProject.screenFixed "屏幕投影"
stroke.alphaTiling "形狀平鋪"
stroke.alphaScale "形狀縮放"
stroke.alphaScale.help "當該值最小時，畫筆形狀將不會大於工具畫筆的半徑圓。"
stroke.alphaMidValue "形狀強度"
stroke.alphaMidValue.help "形狀強度可以讓您自由調節當前筆刷形狀所產生的效果。

當強度值為0時：
- 黑色：無變化
- 白色：凸起

當強度值為0.5時：
- 黑色：下凹
- 白色：凸起

當強度值為1時：
- 黑色：下凹
- 白色：無變化"
// stroke type
stroke.strokeType "筆刷類型"
stroke.strokeTypeDot "點"
stroke.strokeTypeDrag "拖拽"
stroke.strokeTypeGrab "抓取"
stroke.strokeTypeGrabRadius "抓取 - 可調半徑"
stroke.strokeTypeGrabIntensity "抓取 - 可調強度"

// ----------------------------------------------
// symmetry
symmetry "對稱"
symmetry.enable "啟用對稱"
symmetry.plane.title "對稱平面"
symmetry.toolIgnore "當前工具不適用對稱。"
symmetry.radial.title "圓周對稱"
symmetry.radialX "X"
symmetry.radialY "Y"
symmetry.radialZ "Z"
// method
symmetry.method "對稱類型"
symmetry.method.help "- 本體對稱
可使用軸向變換或自由變換等工具移動調整對稱平面。

- 世界對稱
對稱平面是固定不動的。"
symmetry.methodWorld "世界對稱"
symmetry.methodLocal "本體對稱"
// flip
symmetry.flip "翻轉模型"
// mirror
symmetry.mirror "鏡像"
symmetry.mirror.help "嘗試在不影響拓撲的情況下重新應用對稱。

拓撲必須對稱且至少有一個邊緣恰好位於對稱平面上才能成功應用。

如果鏡像失敗，將會建議您強制對稱。但這樣會影響到拓撲。"
symmetry.mirrorLeftToRight "從左至右"
symmetry.mirrorRightToLeft "從右至左"
symmetry.mirrorFail "對稱失敗：

是否使用鏡像來強制對稱？"
symmetry.mirrorUseMasking "保護蒙版區域"
symmetry.mirrorUseMasking.help "啟用後，在鏡像時將不會修改蒙版區域。

該選項會忽略非對稱網格。"
// reset
symmetry.reset "重設對稱中心"
symmetry.resetCenterMesh "對象中心"
symmetry.resetCenterWorld "世界中心"
symmetry.resetDirection "重設鏡像平面"
// advanced
symmetry.showLine "顯示線條"
symmetry.showPlane "顯示平面"
symmetry.editWarning "編輯鏡像平面是實驗性功能。"
symmetry.edit "軸向變換"
symmetry.edit.help "您可以自由改變鏡像平面。

此功能並未完善，請盡量不要使用。"

// ----------------------------------------------
// tools icons on the left (ICON FIT)
tool.dynTopo "動態網格"
tool.symmetry "對稱"
tool.mirror "鏡像"
tool.clay "黏土"
tool.clay.sub "反向"
tool.brush "標準"
tool.move "移動"
tool.move.normal "法線方向"
tool.drag "拖拽"
tool.smooth "平滑"
tool.smooth.relax "規整網格"
tool.mask "蒙版"
tool.mask.unmask "消除蒙版"
tool.maskSelector "選擇蒙版"
tool.smudge "塗抹"
tool.flatten "鏟平"
tool.flatten.fill "填充"
tool.layer "層"
tool.crease "褶皺"
tool.trim "裁切"
tool.split "分割"
tool.project "投射"
tool.inflate "膨脹"
tool.pinch "擠捏"
tool.nudge "觸碰"
tool.stamp "圖章"
tool.clearLayer "擦除"
tool.gizmo "軸向變換"
tool.gizmo.auto "自動原點"
tool.gizmo.editPivot "編輯原點"
tool.gizmo.rotateSnap ""
tool.gizmo.local "軸向"
tool.transform "自由變換"
tool.transform.move "移動"
tool.transform.rotate "旋轉"
tool.transform.scale "縮放"
tool.transform.snap "吸附中點"
tool.measure "測量"
tool.view "瀏覽模式"
tool.lathe "車削"
tool.tube "圓管"
tool.insert "基本體"
tool.shape.flip "翻轉"
tool.shape.view "瀏覽"
tool.shape.lasso "套索"
tool.shape.curve "曲線"
tool.shape.polygon "多邊形"
tool.shape.path "路徑"
tool.shape.rectangle "矩形"
tool.shape.ellipse "橢圓"
tool.shape.line "直線"
tool.shape.closed "閉合"

// popup when editing sliders
tool.sliderRadius "半徑 $0"
tool.sliderIntensity "強度 $0 %"

// ----------------------------------------------
// title
tool.settingsTitle "設置 ($0)"

// ----------------------------------------------
// tool menu
tool.noSettings "該工具無特殊設置。"

// ----------------------------------------------
// clay
tool.clay.flattenOffset "展平偏移 "

// ----------------------------------------------
// crease
tool.crease.pinchFactor "力度"

// ----------------------------------------------
// layer
tool.layer.removeInfluence ""
tool.layer.removeInfluence.help ""
tool.layer.noLayerSelected "此選項僅在選擇圖層後可用。"

// ----------------------------------------------
// flatten
tool.flatten.warning ""
tool.flatten.planeLockOrigin ""
tool.flatten.planeLockNormal ""
tool.flatten.planeAverageOrigin ""
tool.flatten.planeAverageNormal ""
tool.flatten.planeOffset ""

// ----------------------------------------------
// smooth
tool.smooth.stickyBorder ""

// ----------------------------------------------
// paint
tool.paint "繪畫"
tool.paint.erase "橡皮"
tool.paint.depthFilter "深度限定"
tool.paint.layerFilter "圖層限定"
tool.paint.layerFilter.help "啟用該選項後，您只能夠在模型的最底圖層（base）上繪畫。"

// ----------------------------------------------
// masking
tool.mask.clear "清除"
tool.mask.invert "反相"
tool.mask.flipConnected ""
tool.mask.blur "模糊"
tool.mask.sharpen "銳化"
tool.mask.thickness "抽殼厚度"
tool.mask.polish "平滑邊界"
tool.mask.engraveEmboss "凹印"
tool.mask.extract "抽殼"
tool.mask.split "分離"
tool.mask.closeMask "分離操作（蒙版區域）："
tool.mask.closeUnmask "分離操作（非蒙版區域）："
tool.mask.closeAction "分離操作："
tool.mask.closeActionNone "無"
tool.mask.closeActionFill "填補"
tool.mask.closeActionShell "抽殼"
tool.mask.closeActionLayer ""
tool.mask.closeAction.help "- 無
僅分離蒙版區域，並且不封閉對象。

- 填補
孔洞會被填補並光滑。
不要在平面上使用。

- Shell
通過增加厚度的方式來封閉圖形。"

// ----------------------------------------------
// matrix (transform / gizmo)
tool.matrix "坐標"
tool.matrix.clone "克隆"
tool.matrix.action "操作"
tool.matrix.action.help "- 返回原點
將對象移回原位。

- 重設
重設對象的所有變換。

- 烘焙
重新記錄對象變換後的頂點坐標。在視覺上什麽都不會改變。"
tool.matrix.transformOperation "變換操作"
tool.matrix.translation "位移"
tool.matrix.rotation "旋轉"
tool.matrix.scale "縮放"
tool.matrix.uniformScale "等比縮放"
tool.matrix.uniformScale.help "應用不支持非等比縮放的對象變換，因此將用頂點變換替代。"
tool.matrix.moveToOrigin "返回原點"
tool.matrix.resetTransform "重設"
tool.matrix.bakeTransform "烘焙"
tool.matrix.applyMethod "模式："
tool.matrix.applyMethodAuto "自動選擇"
tool.matrix.applyMethodVertex "基於頂點"
tool.matrix.applyMethodObject "基於對象"
tool.matrix.applyMethod.help "- 自動選擇：
讓應用自動選擇兩種模式。
一般會選擇基於對象模式，除非打開了對稱或者在對象上有蒙版。

- 基於頂點：
頂點坐標會獨自變換。
該變換包括對稱與蒙版變換。
如果變換的是基本體，將會強制使用基於對象模式。

- 基於對象：
對象會整體變換。
不會變換對稱與蒙版。
如果進行非等比縮放，將強制使用基於頂點模式。"

// ----------------------------------------------
// transform
tool.transform.multiTouch "多點觸控"
tool.transform.multiTouch.help "如果您禁用此選項，則每次都只能使用移動、旋轉、縮放一種操作。"

// ----------------------------------------------
// gizmo
tool.gizmo.size "部件尺寸"
tool.gizmo.linearRollThreshold "線性旋轉閾值"
tool.gizmo.linearRollThreshold.help "此參數可影響模型旋轉時的“跟手程度”。

當旋轉角度超過設定值後，將會切換到圓形旋轉模式。

如果您更習慣以前的線性旋轉模式（切線方向），可以將角度設置為90°。"
tool.gizmo.autoHide "自動隱藏"
tool.gizmo.tap "單擊改變視圖中心點"
tool.gizmo.tap.help "此選項僅在自定義坐標原點模式下有效（默認禁用）。

- 無
點擊對象後無任何操作。

- 點擊
僅在第一次點擊對象時改變原點。

- 平均
將在圓點坐標設置在前兩次點擊直線的中點。"
tool.gizmo.tapNone "無"
tool.gizmo.tapFirstHit "點擊"
tool.gizmo.tapMiddleStab "中點"

// ----------------------------------------------
// lathe
tool.lathe.axis "旋轉軸位置"
tool.lathe.axis.fixed "固定位置"
tool.lathe.axis.dynamic "自由移動"

// ----------------------------------------------
// tube
tool.tube.snap "表面捕捉"
tool.tube.snap.all "整條曲線"
tool.tube.snap.startEnd "僅端點"

// ----------------------------------------------
// trim
tool.hole "填補孔洞"
tool.hole.fillHoles "填補孔洞"
// tool.hole.reproject "Reproject filled holes"
// tool.hole.reproject.help "Try to reproject the filled hole so that it follows more closely the cut.
// However, it will only work for rather simple projection."
tool.hole.bridges "真實裁切"
tool.hole.bridges.help "啟用此選項後。您可以用裁切的方式在物體上打洞。
裁切效果也會更加接近於您所繪制的形狀。"
tool.hole.threshold "填充閾值"
tool.hole.threshold.help "調整該值以獲得更好的填充效果。"
tool.hole.smoothing "平滑孔洞"

// ----------------------------------------------
// smudge
tool.smudge.quality "質量"
tool.smudge.quality.help "此選項可改變投影的分辨率，將該值調低可提高筆刷速度。"

// ----------------------------------------------
// trim / split / project / selMask
tool.shape "形狀"
tool.shape.rectangleSquare "正方形"
tool.shape.rectangleCentered "中心"
tool.shape.ellipseCircle "圓形"
tool.shape.ellipseCentered "中心"
tool.shape.lineRotateStep "旋轉角度"

// ----------------------------------------------
// measure
tool.measure.goldenRatio "顯示黃金分割比"

// ----------------------------------------------
// topology
topology "拓撲"
// multires
topology.multires.title "多重網格"
topology.multires.title.help "此功能可保留對象的不同分辨率。

您可以在低分辨率對物體進行修改，之後在高分辨率查看並進一步更改細節效果。

圖層在不同分辨率下都可用。"
topology.multiresReverse "簡化"
topology.multiresReverse.confirm "無法再進一步簡化模型。

當前對象的拓撲無法再進一步細分。"
topology.multiresSubdivide "細分"
topology.multiresSubdivideConfirm "該對象將會產生 $0M 個頂點，您確定要繼續嗎？"
topology.multiresDeleteLower "刪除低模"
topology.multiresDeleteHigher "刪除高模"
topology.multiresKeepTriangles "保留三角形"
topology.multiresLinear "平面細分"
topology.multiresLinear.help ""
// voxel
topology.voxel.title "體素網格重構"
topology.voxel.title.help "此功能可重構對象的網格，使其變得更加整齊。

如對象未封閉，則會先填充孔洞。

圖層在應用後會重新投影，但質量會受到影響。"
topology.voxelResolution "分辨率"
topology.voxelRemesh "重構"
topology.voxelSharp ""
topology.voxelSharp.help ""
topology.voxelSubLevel "多重網格等級"
topology.voxelSubLevel.help "此功能可以從重構的結果中生成多個不同分辨率的對象。

能夠在保留較低分辨率對象的同時，擁有更快的運行速度。
如果重構分辨率過高的話，可能會無法生成多重網格或者失去一些細節。"
// dynamic topology
topology.surfaceUniform "重構"
topology.surfaceDetail "分辨率"
topology.surfaceDetail.help "不同於體素網格重構，表面網格重構不需要封閉對象。

此功能還支持遮罩，可以保護您不希望被更改拓撲的部分。

圖層不會受到影響。"
topology.surfaceMethod "模式"
toplogy.surfaceMethodUniformisation "標準"
toplogy.surfaceMethodSubdivision "細分"
toplogy.surfaceMethodDecimation "簡化"
topology.surfaceMethod.help "不同模式的影響：
- 標準：智能判斷
- 細分：增加細節
- 簡化：移除細節"
topology.surfaceUseMasking "保護蒙版區域"
topology.surfaceUseMasking.help "蒙版區域的拓撲將不會受到影響。"
topology.surfaceExtrapolate "頂點擴張"
// dynamic
topology.dynamic "動態網格"
topology.dynamicActivate "啟用"
topology.dynamicActivate.help "啟用此功能可以讓您在雕刻過程中實時增刪網格。

開啟此功能可能會對性能產生較大影響。

圖層不會受到影響。"
topology.dynamicDetailLevel "細節"
topology.dynamicDetailEdge "細節等級"
topology.dynamicDetailMethod "細節等級模式"
topology.dynamicDetailMethodZoom "視野"
topology.dynamicDetailMethodRadius "半徑"
topology.dynamicDetailMethodConstant "網格"
topology.dynamicDetailMethod.help "- 視野
視野縮放程度決定拓撲的詳細程度。

- 半徑
筆刷半徑決定拓撲的詳細程度。

- Constant
細節等級決定拓撲的詳細程度。"
topology.dynamicQuality "質量"
topology.dynamicQuality.help "性能模式特性如下：
- 在雕刻前會對模型進行細分，可以減少您在雕刻過程中產生的偽像。
- 無法逐步應用細化功能，如果您雕刻非常小的細節或進行快速筆觸，則拓撲將始終正確進行細化。

如果您希望使用性能模式，可以考慮在設置面板中開啟“局部雕刻”功能。"
topology.dynamicQualitySpeed "速度"
topology.dynamicQualityQuality "性能"
topology.dynamicUsePressure "同時使用壓感"
topology.dynamicUsePressure.help "啟用此選項後，壓感也會對對象產生影響。"
// topology.dynamicBrush "Brush"
// topology.dynamicGlobal "Global"
// topology.dynamicSettings "Settings - Brush / Global"
// decimate
topology.decimate.title "成型簡化"
topology.decimate.title.help "軟件將會減少模型面數，並把模型轉換為三角面。

此功能可以在保留模型盡可能多細節的同時，減少多邊形的數量。
僅建議在導出為3D打印模型時使用！"
topology.decimate "三角面轉換"
topology.decimateTargetFaces "目標面數"
topology.decimatePaintWeight "繪畫權重"
topology.decimatePaintWeight.help "此功能可以對繪畫的細節邊緣有所保護。

如果您想將模型用於3D打印，可以將該值設置為0。"
topology.decimateUniform "網格相似度"
topology.decimateUniform.help "該參數值越高，簡化輸出的三角面就越相似。"
// topology.decimatePreserveBorders "Preserve borders"
// topology.decimatePreserveBorders.help "Do not decimate the border of the mesh.

// This is only relevant for object that are not watertight."

// BFF is activated through Debug menu
topology.uv.title ""
topology.uvAtlas ""
topology.uvAtlas.warning ""
topology.uvBFF ""
topology.uvBFF.warning ""
topology.uvBFFCones ""
topology.uvBFFCones.help ""
topology.uvDelete ""

// ----------------------------------------------
// version trial
version.buyWeb "該版本僅供演示"
version.buyFull "購買完整版本"
version.trialLimit "試用版本限制：
- 僅允許3次以內的撤銷或重做
- 每個物體僅允許添加一個圖層
- 僅允許啟用一個項目
- 不允許導入和導出"
version.restorePurchase "恢復購買"
version.fullFeatures "- 撤消或重做不受限制
- 圖層數量不受限制
- 允許保存和載入
- 可以導入和導出文件"