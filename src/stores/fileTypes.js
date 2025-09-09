import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useFileTypesStore = defineStore('fileTypes', () => {
  // 默认文件类型配置
  const defaultFileTypes = {
    audio: {
      name: '音频',
      extensions: ['aac', 'ac3', 'aif', 'aifc', 'aiff', 'au', 'cda', 'dts', 'fla', 'flac', 'it', 'm1a', 'm2a', 'm3u', 'm4a', 'm4b', 'm4p', 'mid', 'midi', 'mka', 'mod', 'mp2', 'mp3', 'mpa', 'ogg', 'ra', 'rmi', 'snd', 'spc', 'umx', 'voc', 'wav', 'wma', 'xm'],
      isCustom: false
    },
    video: {
      name: '视频',
      extensions: ['3g2', '3gp', '3gp2', '3gpp', 'amr', 'amv', 'asf', 'avi', 'bdmv', 'bik', 'd2v', 'divx', 'drc', 'dsa', 'dsm', 'dss', 'dsv', 'evo', 'f4v', 'flc', 'fli', 'flic', 'flv', 'hdmov', 'ifo', 'ivf', 'm1v', 'm2p', 'm2t', 'm2ts', 'm2v', 'm4v', 'mkv', 'mp2v', 'mp4', 'mp4v', 'mpe', 'mpeg', 'mpg', 'mpls', 'mpv2', 'mpv4', 'mov', 'mts', 'ogm', 'ogv', 'pss', 'pva', 'qt', 'ram', 'ratdvd', 'rm', 'rmm', 'rmvb', 'roq', 'rpm', 'smil', 'smk', 'swf', 'tp', 'tpr', 'ts', 'vob', 'vp6', 'webm', 'wm', 'wmp', 'wmv'],
      isCustom: false
    },
    pic: {
      name: '图片',
      extensions: ['ani', 'bmp', 'gif', 'ico', 'jpe', 'jpeg', 'jpg', 'pcx', 'png', 'psd', 'tga', 'tif', 'tiff', 'webp', 'wmf'],
      isCustom: false
    },
    doc: {
      name: '文档',
      extensions: ['c', 'chm', 'cpp', 'csv', 'cxx', 'doc', 'docm', 'docx', 'dot', 'dotm', 'dotx', 'h', 'hpp', 'htm', 'html', 'hxx', 'ini', 'java', 'lua', 'mht', 'mhtml', 'odt', 'pdf', 'potx', 'potm', 'ppam', 'ppsm', 'ppsx', 'pps', 'ppt', 'pptm', 'pptx', 'rtf', 'sldm', 'sldx', 'thmx', 'txt', 'vsd', 'wpd', 'wps', 'wri', 'xlam', 'xls', 'xlsb', 'xlsm', 'xlsx', 'xltm', 'xltx', 'xml'],
      isCustom: false
    },
    exe: {
      name: '可执行文件',
      extensions: ['bat', 'cmd', 'exe', 'msi', 'msp', 'scr'],
      isCustom: false
    },
    zip: {
      name: '压缩文件',
      extensions: ['7z', 'ace', 'arj', 'bz2', 'cab', 'gz', 'gzip', 'jar', 'r00', 'r01', 'r02', 'r03', 'r04', 'r05', 'r06', 'r07', 'r08', 'r09', 'r10', 'r11', 'r12', 'r13', 'r14', 'r15', 'r16', 'r17', 'r18', 'r19', 'r20', 'r21', 'r22', 'r23', 'r24', 'r25', 'r26', 'r27', 'r28', 'r29', 'rar', 'tar', 'tgz', 'z', 'zip'],
      isCustom: false
    }
  }

  // 文件类型配置
  const fileTypes = ref({ ...defaultFileTypes })

  // 加载存储的配置
  const loadFileTypes = () => {
    const stored = localStorage.getItem('wem-file-types')
    if (stored) {
      try {
        fileTypes.value = JSON.parse(stored)
      } catch (e) {
        console.error('加载文件类型配置失败:', e)
        fileTypes.value = { ...defaultFileTypes }
      }
    }
  }

  // 保存配置到本地存储
  const saveFileTypes = () => {
    localStorage.setItem('wem-file-types', JSON.stringify(fileTypes.value))
  }

  // 更新文件类型
  const updateFileType = (key, name, extensions) => {
    if (fileTypes.value[key]) {
      fileTypes.value[key].name = name
      fileTypes.value[key].extensions = [...extensions]
      saveFileTypes()
    }
  }

  // 添加自定义文件类型
  const addCustomFileType = (key, name, extensions) => {
    if (!fileTypes.value[key]) {
      fileTypes.value[key] = {
        name,
        extensions: [...extensions],
        isCustom: true
      }
      saveFileTypes()
      return true
    }
    return false
  }

  // 删除自定义文件类型
  const deleteCustomFileType = (key) => {
    if (fileTypes.value[key] && fileTypes.value[key].isCustom) {
      delete fileTypes.value[key]
      saveFileTypes()
      return true
    }
    return false
  }

  // 重置为默认配置
  const resetToDefault = () => {
    fileTypes.value = { ...defaultFileTypes }
    saveFileTypes()
  }

  // 初始化时加载配置
  loadFileTypes()

  return {
    fileTypes,
    updateFileType,
    addCustomFileType,
    deleteCustomFileType,
    resetToDefault,
    saveFileTypes
  }
}, {
  persist: true
})