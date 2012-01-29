# originally from jtaylor: http://paste.ubuntu.com/821868/
def getDownloadCounts(ppa, **kwargs):
  """ binary_name, created_since_date, exact_match, version, distro_arch_series"""
  from collections import defaultdict
  stat = defaultdict(int)
  ppa = lp.me.getPPAByName(name=ppa)
  pub = ppa.getPublishedBinaries(**kwargs)
  for binary in pub:
    c = binary.getDownloadCount()
    if not binary.architecture_specific:
      stat[binary.binary_package_name] = 0
    stat[binary.binary_package_name] += c
    print binary.binary_package_name, binary.binary_package_version, \
          binary.distro_arch_series, c#, binary.architecture_specific
    print(binary.getDailyDownloadTotals(start_date='2012-01-01', end_date='2012-02-01'))
  return stat
